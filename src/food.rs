use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Res, ResMut, Resource, Sprite, Time, Timer, TimerMode};
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
    food_time: Res<FoodTimer>,
) -> bool {
    if food_time.0.finished() {
        true
    } else {
        false
    }
}

pub fn update_food_timer(
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