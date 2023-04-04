use bevy::prelude::*;

use crate::state::game::damage::Damage;
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
