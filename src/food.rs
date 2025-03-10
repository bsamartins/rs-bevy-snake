use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Query, Sprite};
use rand::random;

use crate::{ARENA_HEIGHT, ARENA_WIDTH, Position, Size};

const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

#[derive(Component)]
pub struct Food;

pub fn spawn_food(mut commands: Commands) {
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

pub fn should_spawn_food(
    food: Query<&Food>
) -> bool {
    let food_exists = food.iter().peekable().peek().is_some();
    if !food_exists {
        true
    } else {
        false
    }
}
