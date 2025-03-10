use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, KeyCode, Query, Res, ResMut, Resource, Sprite, Time, Timer, TimerMode, With};

use crate::{Position, Size, SNAKE_HEAD_COLOR};

#[derive(Resource)]
pub struct SnakeTimer(Timer);

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
}

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

pub fn spawn_snake(mut commands: Commands) {
    commands.spawn(
        Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::new(1.0, 1.0)),
    ).insert(SnakeHead {
        direction: Direction::Up
    }).insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
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
    mut head_positions: Query<(&mut Position, &SnakeHead)>,
) {
    if let Some((mut head_pos, head)) = head_positions.iter_mut().next() {
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
        Self(Timer::from_seconds(0.150, TimerMode::Repeating))
    }
}