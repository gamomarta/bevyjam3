use crate::constants::*;
use crate::state::game::bullet::Bullet;
use bevy::prelude::*;

use crate::state::game::damage::Damage;
use crate::state::game::enemy::Enemy;
use crate::state::game::health::Health;
use crate::state::game::side_effect::extra_damage::ExtraDamageSideEffect;
use crate::state::game::side_effect::{SideEffectTrait, SideEffectType};

#[derive(Component)]
pub struct StrengthenSideEffect {
    pub damage: Damage,
}

impl SideEffectTrait for StrengthenSideEffect {
    fn get_type() -> SideEffectType {
        SideEffectType::Bad
    }
    fn get_description() -> String {
        "Enemy gets stronger".to_string()
    }
}

impl Default for StrengthenSideEffect {
    fn default() -> Self {
        StrengthenSideEffect {
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
