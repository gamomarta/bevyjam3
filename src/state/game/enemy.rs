use bevy::prelude::*;
use std::time::Duration;

use crate::assets::Sprites;
use crate::state::game::damage::Damage;
use crate::state::game::health::Health;
use crate::state::game::movement::Velocity;
use crate::state::AppState;
use rand::Rng;

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(8.0, TimerMode::Repeating)))
            .add_system(spawn_enemy.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub(super) struct Enemy;

#[derive(Deref, DerefMut, Resource)]
struct SpawnTimer(Timer);

fn spawn_enemy(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window: Query<&Window>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
) {
    let window = window.single();

    if timer.tick(time.delta()).just_finished() {
        let mut position = Vec3::new(-window.width() / 2.0 - 35.0, 0.0, 0.0);
        let mut overlapping = true;
        while overlapping {
            overlapping = false;
            for enemy_transform in enemies.iter_mut() {
                let distance = (enemy_transform.translation - position).length();
                if distance < 150.0 {
                    // magic loll
                    overlapping = true;
                    position.x -= 350.0;
                    break;
                }
            }
        }
        commands
            .spawn(SpriteBundle {
                texture: sprites.enemy.clone(),
                transform: Transform::from_translation(Vec3::new(
                    -window.width() / 2.0 - 35.0,
                    0.0,
                    0.0,
                ))
                .with_scale(Vec3::splat(0.3)),
                ..Default::default()
            })
            .insert(Velocity::new(10.0, 0.0))
            .insert(Health::new(10.0))
            .insert(Damage::new(3.0))
            .insert(Enemy);
        let rng = &mut rand::thread_rng();
        let delay = rng.gen_range(1.0..3.0); // magic delay lol
        timer.set_duration(Duration::from_secs_f32(delay));
    }
}
