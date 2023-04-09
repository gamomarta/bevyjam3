use crate::constants::*;
use bevy::prelude::*;

use crate::assets::*;
use crate::state::game::bullet::Bullet;
use crate::state::game::damage::Damage;
use rand::Rng;

use crate::state::game::enemy::*;
use crate::state::game::health::Health;
use crate::state::game::hud::popup::display_popup;
use crate::state::game::movement::Velocity;
use crate::state::game::side_effect::*;
use crate::state::GameEntity;

#[derive(Component, Default)]
pub struct Duplicate;

impl SideEffectTrait for Duplicate {
    fn get_type() -> SideEffectType {
        SideEffectType::Bad
    }
    fn get_description() -> String {
        "May cause mitosis".to_string()
    }
}

pub(super) fn apply(
    mut commands: Commands,
    sprites: Res<Sprites>,
    fonts: Res<Fonts>,
    mut enemies: Query<
        (&mut Transform, &Velocity, &Health, &Damage, Option<&Boss>),
        (With<Enemy>, Without<Bullet>),
    >,
    bullets: Query<&Transform, (With<Bullet>, With<Duplicate>)>,
) {
    for (mut enemy_transform, enemy_velocity, enemy_health, enemy_damage, boss) in
        enemies.iter_mut()
    {
        if boss.is_some() {
            continue;
        }
        for bullet_transform in bullets.iter() {
            let mut rng = thread_rng();
            let distance_between_centers =
                (enemy_transform.translation - bullet_transform.translation).length();
            if distance_between_centers <= ENEMY_SIZE + BULLET_SIZE && rng.gen_bool(0.05) {
                display_popup(
                    "Mitosis!",
                    &enemy_transform.translation,
                    &mut commands,
                    fonts.default_font.clone(),
                );
                let translation_offset = ENEMY_SIZE * 1.3 * Vec3::Y;
                commands
                    .spawn(SpriteBundle {
                        texture: sprites.enemy.clone(),
                        transform: Transform::from_translation(
                            enemy_transform.translation - translation_offset,
                        )
                        .with_scale(Vec3::splat(ENEMY_SPRITE_SCALE)),
                        ..Default::default()
                    })
                    .insert(enemy_velocity.clone())
                    .insert(enemy_health.clone())
                    .insert(enemy_damage.clone())
                    .insert(Enemy)
                    .insert(GameEntity);
                enemy_transform.translation += translation_offset;
            }
        }
    }
}
