use bevy::prelude::*;

use crate::state::game::damage::Damage;
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
        "Causes extra damage".to_string()
    }
}

impl Default for ExtraDamageSideEffect {
    fn default() -> Self {
        ExtraDamageSideEffect {
            damage: Damage::new(1.0),
        }
    }
}
