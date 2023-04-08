use bevy::prelude::*;

mod game;
mod game_over;
mod loading;

pub struct State;

impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(loading::Loading)
            .add_plugin(game::Game)
            .add_plugin(game_over::GameOver);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    PreGame,
    Game,
    TowerChoice,
    TowerPlacing,
    GameOver,
}

#[derive(Component)]
pub struct GameEntity;
