use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub enemies_killed: u64,
}
