use bevy::prelude::*;

pub mod layers {
    pub const GOAL_LAYER: f32 = 1.0;
    pub const TOWER_LAYER: f32 = 1.0;
}

pub const TOWER_SPRITE_SCALE: f32 = 0.5;

pub const DEFAULT_SHOOT_RADIUS: f32 = 400.0;

pub const SHOOT_RADIUS_COLOR: Color = Color::Rgba {
    red: 0.3,
    green: 0.0,
    blue: 0.8,
    alpha: 0.6,
};

pub const STARTING_MONEY: u128 = 30;
