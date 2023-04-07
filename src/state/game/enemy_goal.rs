use crate::constants::GOAL_POSITION;
use bevy::prelude::*;

use crate::state::game::damage::Damage;
use crate::state::game::enemy::Enemy;
use crate::state::game::goal::Goal;
use crate::state::game::health::Health;
use crate::state::AppState;

pub(super) struct EnemyGoal;

impl Plugin for EnemyGoal {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_reaches_goal.in_set(OnUpdate(AppState::Game)));
    }
}

fn enemy_reaches_goal(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform, &Damage), With<Enemy>>,
    mut goals: Query<(Entity, &mut Health), With<Goal>>,
) {
    for (enemy, enemy_transform, damage) in enemies.iter() {
        for (goal, mut health) in goals.iter_mut() {
            if enemy_transform.translation.x > GOAL_POSITION {
                *health -= damage;
                commands.entity(enemy).despawn();
                if health.is_dead() {
                    commands.entity(goal).despawn();
                }
            }
        }
    }
}
