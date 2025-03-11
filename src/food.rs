use crate::snake::{SnakeHead, SnakeSegments};
use crate::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::{debug, Commands, Component, IntoSystemConfigs, Query, ResMut, Sprite, With, Without};
use rand::random;
use std::collections::HashSet;

const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food)
            .add_systems(Update, spawn_food.run_if(should_spawn_food));
    }
}

#[derive(Component)]
pub struct Food;

pub fn spawn_food(
    mut commands: Commands,
    head: Query<&mut Position, With<SnakeHead>>,
    segments: ResMut<SnakeSegments>,
    mut positions: Query<&mut Position, Without<SnakeHead>>,
) {
    if head.is_empty() {
        return;
    }
    let head_position = head.single();

    let mut occupied_pos = segments.segments()
        .iter()
        .filter_map(|e| {
            if let Ok(pos) = positions.get_mut(e.to_owned()) {
                Some(pos.to_owned())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    occupied_pos.insert(head_position.to_owned());

    debug!("occupied_positions {:?}", occupied_pos);
    let max_pos = ARENA_WIDTH * ARENA_HEIGHT;
    if (occupied_pos.len() as u32) >= max_pos {
        info!("All positions occupied!!!");
    }

    let mut spawn_pos = Position { x: 0, y: 0 };
    loop {
        spawn_pos = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        };
        if !occupied_pos.contains(&spawn_pos) {
            break;
        }
    }

    commands
        .spawn(
            Sprite::from_color(FOOD_COLOR, Vec2::new(1.0, 1.0)),
        )
        .insert(Food)
        .insert(spawn_pos)
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
