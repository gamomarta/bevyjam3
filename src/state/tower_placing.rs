use bevy::prelude::*;

use crate::assets::Sprites;
use crate::constants::layers::*;
use crate::constants::*;
use crate::state::AppState;
use crate::utils::*;

pub(super) struct TowerPlacing;

impl Plugin for TowerPlacing {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tower_plan.in_schedule(OnEnter(AppState::TowerPlacing)))
            .add_system(update_plan_position.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(click.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(cleanup.in_schedule(OnExit(AppState::TowerPlacing)));
    }
}

#[derive(Component)]
struct TowerPlan;

fn spawn_tower_plan(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn(SpriteBundle {
            texture: sprites.tower.clone(),
            transform: Transform::from_scale(Vec3::splat(TOWER_SPRITE_SCALE)),
            ..Default::default()
        })
        .insert(TowerPlan);
}

fn update_plan_position(
    mut tower_plan: Query<&mut Transform, With<TowerPlan>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let mut tower_plan = tower_plan.single_mut();
    let window = window.single();
    let camera = camera.single();

    let Some(cursor_position) = cursor_coordinates_to_world_coordinates(window, camera) else { return; };

    tower_plan.translation = cursor_position.extend(TOWER_LAYER);
}

fn click(mouse: Res<Input<MouseButton>>, mut next_state: ResMut<NextState<AppState>>) {
    if mouse.just_pressed(MouseButton::Left) {
        next_state.set(AppState::Game);
    }
}

fn cleanup(mut commands: Commands, tower: Query<Entity, With<TowerPlan>>) {
    commands.entity(tower.single()).despawn();
}
