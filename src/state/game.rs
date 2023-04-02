use bevy::prelude::*;

mod enemy;
mod movement;
mod setup;

pub(super) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugin(enemy::EnemyPlugin)
            .add_plugin(movement::Movement)
            .add_plugin(setup::Setup);
    }
}
