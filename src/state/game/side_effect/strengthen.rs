use crate::assets::Fonts;
use crate::constants::*;
use bevy::prelude::*;

use crate::state::game::damage::Damage;
use crate::state::game::enemy::Enemy;
use crate::state::game::goal::Goal;
use crate::state::game::hud::popup::display_popup;
use crate::state::game::side_effect::*;

#[derive(Component)]
pub struct StrengthenSideEffect {
    pub damage: Damage,
}

impl SideEffectTrait for StrengthenSideEffect {
    fn get_type() -> SideEffectType {
        SideEffectType::Bad
    }
    fn get_description() -> String {
        "May increase infection rate".to_string()
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
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut enemies: Query<(&Transform, &mut Damage), With<Enemy>>,
    bullets: Query<(&Transform, &StrengthenSideEffect), With<Goal>>, /*TODO: WHAT? goal, but bullet size?*/
) {
    for (enemy_transform, mut enemy_damage) in enemies.iter_mut() {
        for (bullet_transform, strengthen) in bullets.iter() {
            let distance_between_centers =
                (enemy_transform.translation - bullet_transform.translation).length();
            if distance_between_centers <= ENEMY_SIZE + BULLET_SIZE && !enemy_damage.is_increased()
            {
                display_popup(
                    "More infectious!",
                    &enemy_transform.translation,
                    &mut commands,
                    fonts.default_font.clone(),
                );
                *enemy_damage += &strengthen.damage;
            }
        }
    }
}
