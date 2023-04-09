use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::assets::*;
use crate::constants::layers::*;
use crate::constants::*;
use crate::state::game::enemy::Enemy;
use crate::state::game::money::TowerCost;
use crate::state::game::shooting::*;
use crate::state::game::tower::Tower;
use crate::state::game::tower_choice::TowerCreationEvent;
use crate::state::game::wobble::ShootWobble;
use crate::state::*;
use crate::utils::*;

pub(in crate::state) struct TowerPlacing;

impl Plugin for TowerPlacing {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tower_plan.in_schedule(OnEnter(AppState::TowerPlacing)))
            .add_system(update_plan_position.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(check_enemy_overlap.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(check_goal_overlap.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(check_tower_overlap.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(
                range_color
                    .in_set(OnUpdate(AppState::TowerPlacing))
                    .after(check_enemy_overlap)
                    .after(check_goal_overlap)
                    .after(check_tower_overlap),
            )
            .add_system(
                click
                    .in_set(OnUpdate(AppState::TowerPlacing))
                    .after(check_enemy_overlap)
                    .after(check_goal_overlap)
                    .after(check_tower_overlap),
            )
            .add_system(cleanup.in_schedule(OnExit(AppState::TowerPlacing)));
    }
}

#[derive(Component, Default)]
struct TowerPlan {
    enemy_overlap: bool,
    goal_overlap: bool,
    tower_overlap: bool,
}

impl TowerPlan {
    fn is_valid(&self) -> bool {
        !(self.enemy_overlap || self.goal_overlap || self.tower_overlap)
    }
}

fn spawn_tower_plan(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
    mut tower_creation_event_reader: EventReader<TowerCreationEvent>,
) {
    for tower_creation_event in tower_creation_event_reader.iter() {
        commands
            .spawn(SpriteBundle {
                texture: sprites.tower.clone(),
                transform: Transform::from_scale(Vec3::splat(TOWER_SPRITE_SCALE)),
                ..Default::default()
            })
            .insert(TowerPlan::default())
            .insert(ShootWobble::new())
            .insert(ShootTimer(Timer::from_seconds(0.6, TimerMode::Once)))
            .insert(ShootRadius(DEFAULT_SHOOT_RADIUS))
            .with_children(|tower| {
                tower
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(
                                shape::Circle::new(DEFAULT_SHOOT_RADIUS / TOWER_SPRITE_SCALE)
                                    .into(),
                            )
                            .into(),
                        material: materials.tower_range.clone(),
                        transform: Transform::from_translation(RANGE_LAYER * Vec3::Z),
                        ..default()
                    })
                    .insert(ShootRadiusImage);
            })
            .insert(tower_creation_event.side_effects.clone())
            .insert(GameEntity);
    }
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

fn check_enemy_overlap(
    mut tower_plans: Query<(&mut TowerPlan, &Transform)>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    for (mut tower_plan, tower_transform) in tower_plans.iter_mut() {
        tower_plan.enemy_overlap = enemies.iter().any(|enemy_transform| {
            (enemy_transform.translation - tower_transform.translation).length()
                < TOWER_RADIUS + ENEMY_SIZE
        })
    }
}

fn check_goal_overlap(mut tower_plans: Query<(&mut TowerPlan, &Transform)>) {
    for (mut tower_plan, tower_transform) in tower_plans.iter_mut() {
        tower_plan.goal_overlap = tower_transform.translation.x > GOAL_POSITION - TOWER_RADIUS;
    }
}

fn check_tower_overlap(
    mut tower_plans: Query<(&mut TowerPlan, &Transform)>,
    towers: Query<&Transform, With<Tower>>,
) {
    for (mut tower_plan, tower_transform) in tower_plans.iter_mut() {
        tower_plan.tower_overlap = towers.iter().any(|transform| {
            (transform.translation - tower_transform.translation).length() < TOWER_RADIUS * 2.0
        })
    }
}

fn range_color(
    materials: Res<Materials>,
    tower_plan: Query<(&TowerPlan, &Children)>,
    mut ranges: Query<&mut Handle<ColorMaterial>, With<ShootRadiusImage>>,
) {
    for (tower_plan, children) in tower_plan.iter() {
        for &child in children {
            let Ok(mut range_color) = ranges.get_mut(child) else { continue; };
            *range_color = if tower_plan.is_valid() {
                materials.tower_range.clone()
            } else {
                materials.tower_invalid.clone()
            };
        }
    }
}

fn click(
    mouse: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>,
    tower_plans: Query<&TowerPlan>,
    mut tower_cost: Query<&mut TowerCost>,
) {
    for tower_plan in tower_plans.iter() {
        if !tower_plan.is_valid() {
            continue;
        }
        if mouse.just_pressed(MouseButton::Left) {
            tower_cost.single_mut().increase();
            next_state.set(AppState::Game);
        }
    }
}

fn cleanup(mut commands: Commands, tower: Query<Entity, With<TowerPlan>>) {
    commands
        .entity(tower.single())
        .remove::<TowerPlan>()
        .insert(Tower);
}
