use bevy::prelude::*;

pub const NUMBER_OF_TOWERS_TO_GENERATE: u8 = 3;

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

pub const BUTTON_COLOR: Color = Color::DARK_GRAY;
pub const HOVERED_COLOR: Color = Color::GREEN;
pub const CLICKED_COLOR: Color = Color::LIME_GREEN;
pub const DEACTIVATED_COLOR: Color = Color::ORANGE_RED;
