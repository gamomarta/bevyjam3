use bevy::prelude::*;

use crate::assets::Sprites;
use crate::constants::layers::*;
use crate::constants::*;
use crate::state::AppState;

pub(super) struct Setup;

impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_camera.in_schedule(OnEnter(AppState::PreGame)))
            .add_system(spawn_background.in_schedule(OnEnter(AppState::PreGame)))
            .add_system(start.in_set(OnUpdate(AppState::PreGame)));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_background(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn(SpriteBundle {
        texture: sprites.background.clone(),
        transform: Transform::from_translation(BACKGROUND_LAYER * Vec3::Z)
            .with_scale(Vec3::splat(BACKGROUND_SPRITE_SCALE)),
        ..Default::default()
    });
}

fn start(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Game);
}
