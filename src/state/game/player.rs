use crate::components::Player;
use bevy::prelude::*;

use crate::state::game::money::{Money, TowerCost};
use crate::state::*;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::PreGame)));
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    money: Money,
    price: TowerCost,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default()).insert(GameEntity);
}
