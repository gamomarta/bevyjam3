use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use rand::prelude::*;

use crate::state::AppState;

mod duplicate;
use duplicate::Duplicate;
mod extra_damage;
pub use extra_damage::ExtraDamageSideEffect;
mod insta_kill;
use insta_kill::InstaKill;
mod slow_down;
use slow_down::SlowDown;
mod speed_up;
use speed_up::SpeedUp;
mod strengthen;
use strengthen::StrengthenSideEffect;

pub(super) struct SideEffectPlugin;

impl Plugin for SideEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(duplicate::apply.in_set(OnUpdate(AppState::Game)))
            .add_system(insta_kill::apply.in_set(OnUpdate(AppState::Game)))
            .add_system(slow_down::apply.in_set(OnUpdate(AppState::Game)))
            .add_system(
                speed_up::apply
                    .in_set(OnUpdate(AppState::Game))
                    .after(slow_down::apply),
            )
            .add_system(strengthen::apply.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Clone)]
pub enum SideEffect {
    ExtraDamage,
    Strengthen,

    InstaKill,
    Duplicate,

    SlowDown,
    SpeedUp,
    //TODO: Knockback/Knockforward, Freeze/Teleport, Smallen/Enlarge
}

impl SideEffect {
    pub fn sample<R: Rng>(rng: &mut R, size: usize) -> Vec<Self> {
        vec![
            SideEffect::ExtraDamage,
            SideEffect::Strengthen,
            SideEffect::InstaKill,
            SideEffect::Duplicate,
            SideEffect::SlowDown,
            SideEffect::SpeedUp,
        ]
        .into_iter()
        .choose_multiple(rng, size)
    }
    pub fn insert_into(&self, commands: &mut EntityCommands) {
        match self {
            SideEffect::ExtraDamage => commands.insert(ExtraDamageSideEffect::default()),
            SideEffect::Strengthen => commands.insert(StrengthenSideEffect::default()),
            SideEffect::InstaKill => commands.insert(InstaKill::default()),
            SideEffect::Duplicate => commands.insert(Duplicate::default()),
            SideEffect::SlowDown => commands.insert(SlowDown::default()),
            SideEffect::SpeedUp => commands.insert(SpeedUp::default()),
        };
    }
    pub fn get_type(&self) -> SideEffectType {
        match self {
            SideEffect::ExtraDamage => ExtraDamageSideEffect::get_type(),
            SideEffect::Strengthen => StrengthenSideEffect::get_type(),
            SideEffect::InstaKill => InstaKill::get_type(),
            SideEffect::Duplicate => Duplicate::get_type(),
            SideEffect::SlowDown => SlowDown::get_type(),
            SideEffect::SpeedUp => SpeedUp::get_type(),
        }
    }
    pub fn get_description(&self) -> String {
        match self {
            SideEffect::ExtraDamage => ExtraDamageSideEffect::get_description(),
            SideEffect::Strengthen => StrengthenSideEffect::get_description(),
            SideEffect::InstaKill => InstaKill::get_description(),
            SideEffect::Duplicate => Duplicate::get_description(),
            SideEffect::SlowDown => SlowDown::get_description(),
            SideEffect::SpeedUp => SpeedUp::get_description(),
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
