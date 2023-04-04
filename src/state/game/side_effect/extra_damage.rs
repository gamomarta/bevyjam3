use crate::constants::{BULLET_SIZE, ENEMY_SIZE};
use crate::state::game::bullet::Bullet;
use bevy::prelude::*;

use crate::state::game::damage::Damage;
use crate::state::game::enemy::Enemy;
use crate::state::game::health::Health;
use crate::state::game::side_effect::{SideEffectTrait, SideEffectType};

#[derive(Component)]
pub struct ExtraDamageSideEffect {
    pub damage: Damage,
}

impl SideEffectTrait for ExtraDamageSideEffect {
    fn get_type() -> SideEffectType {
        SideEffectType::Good
    }
    fn get_description() -> String {
        "Tower does extra damage".to_string()
    }
}

impl Default for ExtraDamageSideEffect {
    fn default() -> Self {
        ExtraDamageSideEffect {
            damage: Damage::new(1.0),
        }
    }
}

pub(super) fn apply(
    mut enemies: Query<(&Transform, &mut Health), With<Enemy>>,
    bullets: Query<(&Transform, &ExtraDamageSideEffect), With<Bullet>>,
) {
    for (enemy_transform, mut enemy_health) in enemies.iter_mut() {
        for (bullet_transform, extra_damage) in bullets.iter() {
            let distance_between_centers =
                (enemy_transform.translation - bullet_transform.translation).length();
            if distance_between_centers <= ENEMY_SIZE + BULLET_SIZE {
                *enemy_health -= &extra_damage.damage;
            }
        }
    }
}
