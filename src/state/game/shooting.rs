use bevy::prelude::*;

use crate::assets::Sprites;
use crate::constants::*;
use crate::state::game::bullet::{Bullet, BULLET_SPEED};
use crate::state::game::damage::Damage;
use crate::state::game::enemy::Enemy;
use crate::state::game::health::Health;
use crate::state::game::movement::Velocity;
use crate::state::game::tower::Tower;
use crate::state::game::tower_choice::SideEffects;
use crate::state::game::wobble::ShootWobble;
use crate::state::AppState;

pub(super) struct Shooting;

impl Plugin for Shooting {
    fn build(&self, app: &mut App) {
        app.add_system(shoot.in_set(OnUpdate(AppState::Game)))
            .add_system(enemy_bullet_collision.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct ShootRadius(pub f32);

#[derive(Component)]
pub struct ShootRadiusImage;

#[derive(Component, Deref, DerefMut)]
pub struct ShootTimer(pub Timer);

fn shoot(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut towers: Query<
        (
            &Transform,
            &ShootRadius,
            &mut ShootTimer,
            &mut ShootWobble,
            &SideEffects,
        ),
        With<Tower>,
    >,
    enemies: Query<(&Transform, &Velocity), With<Enemy>>,
) {
    for (
        tower_transform,
        tower_shoot_radius,
        mut tower_shoot_timer,
        mut shoot_wobble,
        side_effects,
    ) in towers.iter_mut()
    {
        if tower_shoot_timer.tick(time.delta()).finished() {
            let distance_to_tower = |enemy_transform: &&Transform| {
                (enemy_transform.translation - tower_transform.translation).length()
            };
            let closest_enemy = enemies
                .iter()
                .filter(|(transform, _velocity)| {
                    distance_to_tower(transform) < **tower_shoot_radius
                })
                .min_by_key(|(transform, _velocity)| {
                    (distance_to_tower(transform) * 100.0) as u128
                });
            if let Some((target_transform, target_velocity)) = closest_enemy {
                // some mathy stuff lol TODO: refactor
                // TODO: consider divisions by zero
                let x1 = target_transform.translation.x;
                let y1 = target_transform.translation.y;
                let v1 = target_velocity;
                let x2 = tower_transform.translation.x;
                let y2 = tower_transform.translation.y;
                let v = BULLET_SPEED;
                let alpha = (x2 - x1) / (y2 - y1);
                let beta = v1.x() - alpha * v1.y();
                // second degree equation solution
                let a = 1.0 + alpha * alpha;
                let b = -2.0 * beta;
                let c = (beta + alpha * v) * (beta - alpha * v);
                let delta = b * b - 4.0 * a * c;
                let vx_solution1 = (-b - delta.sqrt()) / (2.0 * a);
                let vx_solution2 = (-b + delta.sqrt()) / (2.0 * a);
                let vx = if x1 < x2 { vx_solution1 } else { vx_solution2 };
                let vy_solution1 = -((v * v - vx * vx).sqrt());
                let vy_solution2 = (v * v - vx * vx).sqrt();
                let vy = if y1 < y2 { vy_solution1 } else { vy_solution2 };
                let velocity = Velocity::new(vx, vy);
                shoot_wobble.unpause();
                tower_shoot_timer.reset();
                let mut bullet = commands.spawn(SpriteBundle {
                    texture: sprites.bullet.clone(),
                    transform: Transform::from_translation(tower_transform.translation)
                        .with_scale(Vec3::splat(BULLET_SPRITE_SCALE)),
                    ..Default::default()
                });
                bullet
                    .insert(velocity)
                    .insert(Damage::new(1.0))
                    .insert(Bullet);
                for side_effect in side_effects.iter() {
                    side_effect.insert_into(&mut bullet);
                }
            }
        }
    }
}

fn enemy_bullet_collision(
    mut commands: Commands,
    mut enemies: Query<(&Transform, &mut Health), With<Enemy>>,
    bullets: Query<(Entity, &Transform, &Damage), With<Bullet>>,
) {
    for (enemy_transform, mut enemy_health) in enemies.iter_mut() {
        for (bullet, bullet_transform, bullet_damage) in bullets.iter() {
            let distance_between_centers =
                (enemy_transform.translation - bullet_transform.translation).length();
            if distance_between_centers <= ENEMY_SIZE + BULLET_SIZE {
                commands.entity(bullet).despawn();
                *enemy_health -= bullet_damage;
            }
        }
    }
}
