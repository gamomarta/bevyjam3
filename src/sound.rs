use bevy::prelude::*;

use crate::assets::Music;
use crate::state::AppState;

pub struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app.add_system(game_music.in_schedule(OnEnter(AppState::PreGame)));
    }
}

fn game_music(music: Res<Music>, audio: Res<Audio>) {
    audio.play(music.main_theme.clone());
}
