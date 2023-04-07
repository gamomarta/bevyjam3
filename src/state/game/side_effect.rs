use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::state::AppState;

mod duplicate;
use duplicate::Duplicate;
mod extra_damage;
use extra_damage::ExtraDamageSideEffect;
mod insta_kill;
use insta_kill::InstaKill;
mod strengthen;
use strengthen::StrengthenSideEffect;

pub(super) struct SideEffectPlugin;

impl Plugin for SideEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(duplicate::apply.in_set(OnUpdate(AppState::Game)))
            .add_system(extra_damage::apply.in_set(OnUpdate(AppState::Game)))
            .add_system(insta_kill::apply.in_set(OnUpdate(AppState::Game)))
            .add_system(strengthen::apply.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Clone)]
pub enum SideEffect {
    ExtraDamage,
    Strengthen,

    InstaKill,
    Duplicate,
    //TODO: Knockback/Knockforward, SlowDown/SpeedUp,
}

impl SideEffect {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => SideEffect::ExtraDamage,
            1 => SideEffect::Strengthen,
            2 => SideEffect::InstaKill,
            3 => SideEffect::Duplicate,
            _ => unreachable!(),
        }
    }
    pub fn insert_into(&self, commands: &mut EntityCommands) {
        match self {
            SideEffect::ExtraDamage => commands.insert(ExtraDamageSideEffect::default()),
            SideEffect::Strengthen => commands.insert(StrengthenSideEffect::default()),
            SideEffect::InstaKill => commands.insert(InstaKill::default()),
            SideEffect::Duplicate => commands.insert(Duplicate::default()),
        };
    }
    pub fn get_type(&self) -> SideEffectType {
        match self {
            SideEffect::ExtraDamage => ExtraDamageSideEffect::get_type(),
            SideEffect::Strengthen => StrengthenSideEffect::get_type(),
            SideEffect::InstaKill => InstaKill::get_type(),
            SideEffect::Duplicate => Duplicate::get_type(),
        }
    }
    pub fn get_description(&self) -> String {
        match self {
            SideEffect::ExtraDamage => ExtraDamageSideEffect::get_description(),
            SideEffect::Strengthen => StrengthenSideEffect::get_description(),
            SideEffect::InstaKill => InstaKill::get_description(),
            SideEffect::Duplicate => Duplicate::get_description(),
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
