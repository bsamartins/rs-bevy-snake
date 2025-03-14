use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::food::{Food, FoodPlugin};
use crate::snake::{snake_movement, spawn_snake, SnakePlugin, SnakeSegment, SnakeSegments};

mod snake;
mod food;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn main() {
    App::new()
        .add_event::<GameOverEvent>()
        .add_systems(PreStartup, initialize_window)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, game_over.after(snake_movement))
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_plugins(DefaultPlugins)
        .add_plugins(SnakePlugin)
        .add_plugins(FoodPlugin)
        .run();
}

#[derive(PartialEq, Copy, Clone, Debug, Event)]
struct GameOverEvent;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component, Debug, Hash, Clone, Copy, PartialEq, Eq)]
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

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.read().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}
