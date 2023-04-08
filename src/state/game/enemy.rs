use bevy::prelude::*;
use std::time::Duration;

use crate::assets::Sprites;
use crate::components::Player;
use crate::constants::*;
use crate::state::game::damage::Damage;
use crate::state::game::health::Health;
use crate::state::game::money::Money;
use crate::state::game::movement::Velocity;
use crate::state::*;
use rand::Rng;

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_system(spawn_enemy.in_set(OnUpdate(AppState::Game)))
            .add_system(vertical_bounds.in_set(OnUpdate(AppState::Game)))
            .add_system(enemy_death.in_set(OnUpdate(AppState::Game)))
            .add_system(enemy_despawn.in_set(OnUpdate(AppState::Game)));
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
) {
    let window = window.single();
    let mut rng = rand::thread_rng();

    if timer.tick(time.delta()).just_finished() {
        commands
            .spawn(SpriteBundle {
                texture: sprites.enemy.clone(),
                transform: Transform::from_translation(Vec3::new(
                    -window.width() / 2.0 - ENEMY_SIZE,
                    rng.gen_range(
                        -window.height() / 2.0 + ENEMY_SIZE..window.height() / 2.0 - ENEMY_SIZE,
                    ),
                    layers::ENEMY_LAYER + rng.gen_range(-0.9..0.9),
                ))
                .with_scale(Vec3::splat(ENEMY_SPRITE_SCALE)),
                ..Default::default()
            })
            .insert(Velocity::new(ENEMY_SPEED, 0.0))
            .insert(Health::new(ENEMY_HEALTH))
            .insert(Damage::new(ENEMY_DAMAGE))
            .insert(Enemy)
            .insert(GameEntity);
        let delay = rng.gen_range(1.0..3.0); // magic delay lol
        timer.set_duration(Duration::from_secs_f32(delay));
    }
}

fn vertical_bounds(window: Query<&Window>, mut enemies: Query<&mut Transform, With<Enemy>>) {
    let bound = window.single().height() / 2.0 - ENEMY_SIZE * 2.0;

    for mut enemy_transform in enemies.iter_mut() {
        enemy_transform.translation.y = enemy_transform.translation.y.clamp(-bound, bound);
    }
}

fn enemy_death(
    sprites: Res<Sprites>,
    mut enemies: Query<(&mut Handle<Image>, &mut Sprite, &mut Velocity, &Health), With<Enemy>>,
) {
    for (mut enemy_image, mut enemy_sprite, mut enemy_velocity, enemy_health) in enemies.iter_mut()
    {
        if enemy_health.is_dead() {
            *enemy_image = sprites.defeated_enemy.clone();
            enemy_sprite.flip_x = true;
            // enemy_sprite.color = Color::Rgba {
            //     red: 0.2,
            //     green: 0.3,
            //     blue: 1.0,
            //     alpha: 1.0,
            // };
            *enemy_velocity = Velocity::new(-1000.0, 0.0);
        }
    }
}

fn enemy_despawn(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut players: Query<(&mut Player, &mut Money)>,
) {
    let (mut player, mut money) = players.single_mut();
    for (enemy, enemy_transform) in enemies.iter() {
        if enemy_transform.translation.x < -WINDOW_WIDTH {
            player.enemies_killed += 1;
            *money += Money::for_killing_enemy(); //TODO: enemy should have its own money component
            commands.entity(enemy).despawn();
        }
    }
}
