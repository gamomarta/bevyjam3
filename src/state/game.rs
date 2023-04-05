use bevy::prelude::*;

mod bullet;
mod damage;
mod enemy;
mod enemy_goal;
mod goal;
mod health;
mod hud;
mod money;
mod movement;
mod player;
mod setup;
mod shooting;
mod side_effect;
pub mod sound;
mod tower;
mod tower_choice;
mod tower_placing;

pub(super) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugin(enemy::EnemyPlugin)
            .add_plugin(enemy_goal::EnemyGoal)
            .add_plugin(goal::GoalPlugin)
            .add_plugin(hud::Hud)
            .add_plugin(movement::Movement)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(setup::Setup)
            .add_plugin(shooting::Shooting)
            .add_plugin(side_effect::SideEffectPlugin)
            .add_plugin(sound::Sound)
            .add_plugin(tower_choice::TowerChoice)
            .add_plugin(tower_placing::TowerPlacing)
            .add_plugin(tower::TowerPlugin);
    }
}
