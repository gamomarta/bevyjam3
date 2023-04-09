use bevy::prelude::*;

use crate::constants::*;
use crate::state::game::enemy::Enemy;
use crate::state::game::tower::Tower;
use crate::state::AppState;

pub(super) struct EnemyTower;

impl Plugin for EnemyTower {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_reaches_tower.in_set(OnUpdate(AppState::Game)));
    }
}

fn enemy_reaches_tower(
    mut commands: Commands,
    towers: Query<(Entity, &Transform), With<Tower>>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    for (tower, tower_transform) in towers.iter() {
        for enemy_transform in enemies.iter() {
            if (tower_transform.translation - enemy_transform.translation).length()
                < TOWER_RADIUS + ENEMY_SIZE
            {
                commands.entity(tower).despawn_recursive();
            }
        }
    }
}
