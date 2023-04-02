use bevy::prelude::*;

use crate::assets::Sprites;
use crate::state::game::movement::Velocity;
use crate::state::AppState;

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_system(spawn_enemy.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Deref, DerefMut, Resource)]
struct SpawnTimer(Timer);

fn spawn_enemy(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window: Query<&Window>,
) {
    let window = window.single();

    if timer.tick(time.delta()).just_finished() {
        commands
            .spawn(SpriteBundle {
                texture: sprites.enemy.clone(),
                transform: Transform::from_translation(Vec3::new(-window.width() / 2.0, 0.0, 0.0))
                    .with_scale(Vec3::splat(0.3)),
                ..Default::default()
            })
            .insert(Velocity::new(10.0, 0.0))
            .insert(Enemy);
    }
}
