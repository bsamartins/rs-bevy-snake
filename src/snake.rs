use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, KeyCode, Query, Res, Sprite, With};
use crate::{Position, Size, SNAKE_HEAD_COLOR};

#[derive(Component)]
pub struct SnakeHead;

pub fn spawn_snake(mut commands: Commands) {
    commands.spawn(
        Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::new(1.0, 1.0)),
    ).insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

pub fn snake_movement(
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
