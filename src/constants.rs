use bevy::prelude::*;

pub const NUMBER_OF_TOWERS_TO_GENERATE: u8 = 3;

pub mod layers {
    pub const GOAL_LAYER: f32 = 1.0;
    pub const TOWER_LAYER: f32 = 1.0;
}

pub const TOWER_SPRITE_SCALE: f32 = 0.5;
pub const BULLET_SIZE: f32 = 5.0; //TODO: this is a guess
pub const ENEMY_SIZE: f32 = 6.0 * BULLET_SIZE; //TODO: also a guess
pub const GOAL_SIZE: f32 = 100.0; //TODO: also also a guess
pub const TOWER_SIZE: f32 = 75.0; //TODO: a guess if I ever saw one

pub const DEFAULT_SHOOT_RADIUS: f32 = 400.0;

pub const SHOOT_RADIUS_COLOR: Color = Color::Rgba {
    red: 0.3,
    green: 0.0,
    blue: 0.8,
    alpha: 0.6,
};
pub const TOWER_INVALID_COLOR: Color = Color::Rgba {
    red: 0.8,
    green: 0.0,
    blue: 0.3,
    alpha: 0.6,
};

pub const STARTING_MONEY: u128 = 30;

pub const BUTTON_COLOR: Color = Color::DARK_GRAY;
pub const HOVERED_COLOR: Color = Color::GREEN;
pub const CLICKED_COLOR: Color = Color::LIME_GREEN;
pub const DEACTIVATED_COLOR: Color = Color::ORANGE_RED;
