use crate::constants::*;
use bevy::prelude::*;

use crate::state::game::bullet::Bullet;
use rand::Rng;

use crate::state::game::enemy::Enemy;
use crate::state::game::health::Health;
use crate::state::game::side_effect::*;

#[derive(Component, Default)]
pub struct InstaKill;

impl SideEffectTrait for InstaKill {
    fn get_type() -> SideEffectType {
        SideEffectType::Good
    }
    fn get_description() -> String {
        "May cure instantly".to_string()
    }
}

pub(super) fn apply(
    mut enemies: Query<(&Transform, &mut Health), With<Enemy>>,
    bullets: Query<&Transform, (With<Bullet>, With<InstaKill>)>,
) {
    for (enemy_transform, mut enemy_health) in enemies.iter_mut() {
        for bullet_transform in bullets.iter() {
            let mut rng = rand::thread_rng();
            let distance_between_centers =
                (enemy_transform.translation - bullet_transform.translation).length();
            if distance_between_centers <= ENEMY_SIZE + BULLET_SIZE && rng.gen_bool(0.05) {
                enemy_health.die();
            }
        }
    }
}
