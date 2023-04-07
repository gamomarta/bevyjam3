use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::assets::Materials;
use crate::constants::layers::*;
use crate::constants::*;
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

fn spawn_goal(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, materials: Res<Materials>) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Quad::new(Vec2::new(GOAL_WIDTH, WINDOW_HEIGHT)) //TODO
                        .into(),
                )
                .into(),
            material: materials.goal.clone(),
            transform: Transform::from_translation(Vec3::new(
                GOAL_POSITION + GOAL_WIDTH / 2.0,
                0.0,
                GOAL_LAYER,
            )),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(Health::new(GOAL_HEALTH))
        .insert(Goal);
}

fn lose(goals: Query<With<Goal>>, mut next_state: ResMut<NextState<AppState>>) {
    if goals.is_empty() {
        next_state.set(AppState::GameOver);
    }
}
