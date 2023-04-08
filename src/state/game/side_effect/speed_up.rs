use crate::assets::Fonts;
use crate::constants::*;
use bevy::prelude::*;

use crate::state::game::bullet::Bullet;

use crate::state::game::enemy::Enemy;
use crate::state::game::hud::popup::display_popup;
use crate::state::game::movement::Velocity;
use crate::state::game::side_effect::*;

#[derive(Component, Default)]
pub struct SpeedUp;

impl SideEffectTrait for SpeedUp {
    fn get_type() -> SideEffectType {
        SideEffectType::Bad
    }
    fn get_description() -> String {
        "Strengthens leg muscles".to_string()
    }
}

pub(super) fn apply(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
    bullets: Query<&Transform, (With<Bullet>, With<SpeedUp>)>,
) {
    for (enemy_transform, mut enemy_velocity) in enemies.iter_mut() {
        for bullet_transform in bullets.iter() {
            let distance_between_centers =
                (enemy_transform.translation - bullet_transform.translation).length();
            if distance_between_centers <= ENEMY_SIZE + BULLET_SIZE
                && !enemy_velocity.is_fast()
                && !enemy_velocity.is_slow()
            {
                display_popup(
                    "Fast!",
                    &enemy_transform.translation,
                    &mut commands,
                    fonts.default_font.clone(),
                );
                enemy_velocity.set_speed(FAST_ENEMY_SPEED);
            }
        }
    }
}
