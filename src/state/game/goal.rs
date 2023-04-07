use bevy::prelude::*;

use crate::assets::Sprites;
use crate::constants::layers::GOAL_LAYER;
use crate::constants::GOAL_SPRITE_SCALE;
use crate::state::game::health::Health;
use crate::state::AppState;

pub(super) struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_goal.in_schedule(OnEnter(AppState::PreGame)))
            .add_system(lose.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Goal;

fn spawn_goal(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(500.0, 0.0, GOAL_LAYER))
                .with_scale(Vec3::splat(GOAL_SPRITE_SCALE)),
            texture: sprites.goal.clone(),
            ..Default::default()
        })
        .insert(Health::new(10.0))
        .insert(Goal);
}

fn lose(goals: Query<With<Goal>>, mut next_state: ResMut<NextState<AppState>>) {
    if goals.is_empty() {
        next_state.set(AppState::GameOver);
    }
}
