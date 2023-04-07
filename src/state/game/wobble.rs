use crate::constants::TOWER_WOBBLE_DURATION;
use crate::state::game::bullet::Bullet;
use crate::state::game::enemy::Enemy;
use crate::state::game::tower::Tower;
use crate::state::AppState;
use bevy::prelude::*;

pub(super) struct Wobble;

impl Plugin for Wobble {
    fn build(&self, app: &mut App) {
        app.add_system(shoot_wobble.in_set(OnUpdate(AppState::Game)))
            .add_system(bullet_rotation.in_set(OnUpdate(AppState::Game)))
            .add_system(enemy_walk.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct ShootWobble(Timer);

impl ShootWobble {
    pub fn new() -> Self {
        let mut timer = Timer::from_seconds(TOWER_WOBBLE_DURATION, TimerMode::Once);
        timer.pause();
        Self(timer)
    }
}

fn shoot_wobble(
    mut towers: Query<(&mut Transform, &mut ShootWobble), With<Tower>>,
    time: Res<Time>,
) {
    for (mut tower_transform, mut wobble) in towers.iter_mut() {
        if wobble.paused() {
            continue;
        }
        wobble.tick(time.delta());
        tower_transform.rotation = if wobble.finished() {
            wobble.reset();
            wobble.pause();
            Quat::from_axis_angle(Vec3::Z, 0.0)
        } else {
            Quat::from_axis_angle(Vec3::Z, -wobble.elapsed_secs())
        };
    }
}

fn bullet_rotation(mut bullets: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut transform in bullets.iter_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::Z, 3.0 * time.elapsed_seconds());
    }
}

fn enemy_walk(mut enemies: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    for mut transform in enemies.iter_mut() {
        transform.rotation =
            Quat::from_axis_angle(Vec3::Z, 0.1 * (7.0 * time.elapsed_seconds()).sin());
    }
}
