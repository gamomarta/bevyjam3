use bevy::prelude::*;

mod setup;

pub(super) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugin(setup::Setup);
    }
}
