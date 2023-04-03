use bevy::prelude::*;

mod game;
mod loading;
mod tower_placing;

pub struct State;

impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(loading::Loading)
            .add_plugin(game::Game)
            .add_plugin(tower_placing::TowerPlacing);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    PreGame,
    Game,
    TowerPlacing,
}
