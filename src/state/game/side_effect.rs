use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use rand_derive::Rand;

use crate::state::AppState;

mod extra_damage;
use extra_damage::ExtraDamageSideEffect;
mod strengthen;
use strengthen::StrengthenSideEffect;

pub(super) struct SideEffectPlugin;

impl Plugin for SideEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(extra_damage::apply.in_set(OnUpdate(AppState::Game)))
            .add_system(strengthen::apply.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Clone, Rand)]
pub enum SideEffect {
    ExtraDamage,
    Strengthen,
}

impl SideEffect {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..2) {
            0 => SideEffect::ExtraDamage,
            1 => SideEffect::Strengthen,
            _ => unreachable!(),
        }
    }
    pub fn insert_into(&self, commands: &mut EntityCommands) {
        match self {
            SideEffect::ExtraDamage => commands.insert(ExtraDamageSideEffect::default()),
            SideEffect::Strengthen => commands.insert(StrengthenSideEffect::default()),
        };
    }
    pub fn get_type(&self) -> SideEffectType {
        match self {
            SideEffect::ExtraDamage => ExtraDamageSideEffect::get_type(),
            SideEffect::Strengthen => StrengthenSideEffect::get_type(),
        }
    }
    pub fn get_description(&self) -> String {
        match self {
            SideEffect::ExtraDamage => ExtraDamageSideEffect::get_description(),
            SideEffect::Strengthen => StrengthenSideEffect::get_description(),
        }
    }
}

pub enum SideEffectType {
    Good,
    Bad,
}

pub trait SideEffectTrait {
    fn get_type() -> SideEffectType;
    fn get_description() -> String;
}