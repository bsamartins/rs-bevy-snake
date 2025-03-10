use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

fn main() {
    App::new()
        .insert_resource(FoodTimer::new())
        .add_systems(FixedUpdate, update_food_timer)
        .add_systems(PreStartup, initialize_window)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_systems(Update, food_spawner.run_if(should_spawn_food))
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct SnakeHead;

fn spawn_snake(mut commands: Commands) {
    commands.spawn(
        Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::new(1.0, 1.0)),
    ).insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            pos.y += 1;
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn size_scaling(window_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)>) {
    let window = window_query.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        let x = sprite_size.width / ARENA_WIDTH as f32 * window.width();
        let y = sprite_size.height / ARENA_HEIGHT as f32 * window.height();
        debug!("{x}x{y}, {}x{}", window.width(), window.height());
        transform.scale = Vec3::new(
            x,
            y,
            0.0,
        );
    }
}

fn position_translation(window_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = window_query.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn initialize_window(mut window: Single<&mut Window>) {
    window.title = "snake".to_string();
    window.resolution.set_physical_resolution(500, 500);
}

#[derive(Component)]
struct Food;

fn food_spawner(mut commands: Commands) {
    commands
        .spawn(
            Sprite::from_color(FOOD_COLOR, Vec2::new(1.0, 1.0)),
        )
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn should_spawn_food(
    time: Res<Time>,
    food_time: Res<FoodTimer>,
) -> bool {
    // food_time.0.tick(time.delta());
    if food_time.0.finished() {
        // food_time.0.reset();
        true
    } else {
        false
    }
}

fn update_food_timer(
    time: Res<Time>,
    mut food_time: ResMut<FoodTimer>,
) {
    food_time.0.tick(time.delta());
}

#[derive(Resource)]
pub struct FoodTimer(Timer);

impl FoodTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}