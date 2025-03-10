use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Entity, KeyCode, Query, Res, ResMut, Resource, Sprite, Time, Timer, TimerMode, With};

use crate::{Position, Size};

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

#[derive(Resource)]
pub struct SnakeTimer(Timer);

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Resource)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(
        vec![
            commands.spawn(
                Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::new(1.0, 1.0)),
            ).insert(SnakeHead {
                direction: Direction::Up
            }).insert(Position { x: 3, y: 3 })
                .insert(SnakeSegment)
                .insert(Size::square(0.8))
                .id(),
            spawn_segment(commands, Position { x: 3, y: 2 }),
        ]
    );
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(Sprite::from_color(SNAKE_SEGMENT_COLOR, Vec2::new(1.0, 1.0)))
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub fn snake_movement_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments.0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

pub fn should_move_snake(
    food_time: Res<SnakeTimer>,
) -> bool {
    if food_time.0.finished() {
        true
    } else {
        false
    }
}

pub fn update_snake_timer(
    time: Res<Time>,
    mut food_time: ResMut<SnakeTimer>,
) {
    food_time.0.tick(time.delta());
}

impl SnakeTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.500, TimerMode::Repeating))
    }
}