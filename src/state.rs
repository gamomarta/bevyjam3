use bevy::prelude::*;

mod game;
mod loading;

pub struct State;

impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(loading::Loading)
            .add_plugin(game::Game);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}
