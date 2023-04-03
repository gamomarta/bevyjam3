use bevy::prelude::*;

use crate::state::AppState;

mod velocity;
pub use velocity::*;

pub(super) struct Movement;

impl Plugin for Movement {
    fn build(&self, app: &mut App) {
        app.add_system(movement.in_set(OnUpdate(AppState::Game)));
    }
}

pub fn movement(time: Res<Time>, mut enemies: Query<(&mut Transform, &Velocity)>) {
    for (mut enemy_transform, velocity) in enemies.iter_mut() {
        enemy_transform.translation += velocity * time.delta();
    }
}
