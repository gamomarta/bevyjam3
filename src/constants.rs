use bevy::prelude::*;

pub const NUMBER_OF_TOWERS_TO_GENERATE: u8 = 3;

pub mod layers {
    pub const GOAL_LAYER: f32 = 1.0;
    pub const TOWER_LAYER: f32 = 1.0;
}

pub const TOWER_SPRITE_RADIUS: f32 = 1500.0 / 2.0;
pub const TOWER_SPRITE_SCALE: f32 = 0.1;
pub const TOWER_RADIUS: f32 = TOWER_SPRITE_RADIUS * TOWER_SPRITE_SCALE;

pub const BULLET_SPRITE_RADIUS: f32 = 1360.0 / 2.0;
pub const BULLET_SPRITE_SCALE: f32 = 0.03;
pub const BULLET_SIZE: f32 = BULLET_SPRITE_RADIUS * BULLET_SPRITE_SCALE;

pub const ENEMY_SIZE: f32 = 30.0; //TODO: also a guess
pub const GOAL_SIZE: f32 = 100.0; //TODO: also also a guess

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
