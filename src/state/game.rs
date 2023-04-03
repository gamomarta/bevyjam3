use bevy::prelude::*;

mod bullet;
mod damage;
mod enemy;
mod health;
mod hud;
mod money;
mod movement;
mod player;
mod setup;
mod shooting;
mod tower;
pub mod tower_placing;
pub mod tower_radius;

pub(super) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugin(enemy::EnemyPlugin)
            .add_plugin(hud::Hud)
            .add_plugin(movement::Movement)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(setup::Setup)
            .add_plugin(shooting::Shooting)
            .add_plugin(tower::TowerPlugin);
    }
}
