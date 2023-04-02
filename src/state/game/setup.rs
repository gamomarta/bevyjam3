use bevy::prelude::*;

use crate::assets::Sprites;
use crate::state::AppState;

pub(super) struct Setup;

impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_camera.in_schedule(OnEnter(AppState::Game)))
            .add_system(spawn_bevy_logo.in_schedule(OnEnter(AppState::Game)));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_bevy_logo(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn(SpriteBundle {
        texture: sprites.bevy_logo.clone(),
        ..Default::default()
    });
}