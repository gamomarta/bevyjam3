use bevy::prelude::*;

mod bullet;
mod enemy;
mod movement;
mod setup;
mod shooting;
mod tower;

pub(super) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugin(enemy::EnemyPlugin)
            .add_plugin(movement::Movement)
            .add_plugin(setup::Setup)
            .add_plugin(shooting::Shooting)
            .add_plugin(tower::TowerPlugin);
    }
}
