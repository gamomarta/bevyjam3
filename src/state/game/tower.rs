use bevy::prelude::*;

use crate::assets::Sprites;
use crate::state::AppState;

pub(super) struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tower.in_schedule(OnEnter(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Tower;

#[derive(Component, Deref, DerefMut)]
pub struct ShootTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ShootRadius(f32);

fn spawn_tower(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn(SpriteBundle {
            texture: sprites.tower.clone(),
            transform: Transform::from_translation(Vec3::new(-100.0, 250.0, 0.0))
                .with_scale(Vec3::splat(0.5)),
            ..Default::default()
        })
        .insert(ShootTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .insert(ShootRadius(600.0))
        .insert(Tower);
}
