use bevy::prelude::*;

// use crate::assets::*;
use crate::state::AppState;

pub struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app.add_system(game_music.in_schedule(OnEnter(AppState::PreGame)));
    }
}

fn game_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("main_theme.ogg"));
}

// TODO: make this work
// fn game_music(music: Res<Music>, audio: Res<Audio>) {
//     audio.play(music.main_theme.clone());
// }
