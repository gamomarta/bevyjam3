use bevy::prelude::*;

use crate::assets::Music;
use crate::state::AppState;

pub(super) struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app.add_system(game_music.in_schedule(OnEnter(AppState::PreGame)))
            .add_system(game_over_music.in_schedule(OnEnter(AppState::GameOver)));
    }
}

fn game_music(mut music: ResMut<Music>, audio: Res<Audio>, audio_sinks: Res<Assets<AudioSink>>) {
    if let Some(sink) = audio_sinks.get(&music.game_over_handle) {
        sink.stop();
    }
    let weak_handle = audio.play_with_settings(music.main_theme.clone(), PlaybackSettings::LOOP);
    music.main_theme_handle = audio_sinks.get_handle(&weak_handle);
}

fn game_over_music(
    mut music: ResMut<Music>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    if let Some(sink) = audio_sinks.get(&music.main_theme_handle) {
        sink.stop();
    }
    let weak_handle = audio.play_with_settings(music.game_over.clone(), PlaybackSettings::LOOP);
    music.game_over_handle = audio_sinks.get_handle(&weak_handle);
}
