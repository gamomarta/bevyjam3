use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::assets::*;
use crate::constants::layers::*;
use crate::constants::*;
use crate::state::game::shooting::*;
use crate::state::game::tower::Tower;
use crate::state::game::tower_choice::TowerCreationEvent;
use crate::state::AppState;
use crate::utils::*;

pub(in crate::state) struct TowerPlacing;

impl Plugin for TowerPlacing {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tower_plan.in_schedule(OnEnter(AppState::TowerPlacing)))
            .add_system(update_plan_position.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(click.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(validate_position.in_set(OnUpdate(AppState::TowerPlacing)))
            .add_system(cleanup.in_schedule(OnExit(AppState::TowerPlacing)));
    }
}

#[derive(Component)]
struct TowerPlan;

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
            .insert(TowerPlan)
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
                        transform: Transform::from_translation(2.0 * Vec3::Z),
                        ..default()
                    })
                    .insert(ShootRadiusImage);
            })
            .insert(tower_creation_event.side_effects.clone());
        // TODO: move to OnUpdate(AppState::TowerPlacing)
        // commented because of borrow checker issues, and because it will be moved
        // check if towere intersects with the enemy path
        // let mut is_path_obstructed = false;
        // if let Ok(tower_transform) = query.get_component_mut::<Transform>(tower_entity.id()) {
        //     for enemy_transform in enemies_query.iter() {
        //         // should check the circule probably
        //         let distance_to_enemy =
        //             enemy_transform.translation.x - tower_transform.translation.x;
        //         if distance_to_enemy < 500.0 {
        //             //magic lol
        //             is_path_obstructed = true;
        //             break;
        //         }
        //     }
        // }
        // if is_path_obstructed {
        //     commands.entity(tower_entity.id()).despawn();
        //     //get the money backz
        // }
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

fn validate_position(// mut radii: Query<&mut Handle<ColorMaterial>, With<ShootRadius>>,
) {
    // for mut radius_color in radii.iter_mut() {
    //     *radius_color = Color::RED;
    // }
}

fn click(mouse: Res<Input<MouseButton>>, mut next_state: ResMut<NextState<AppState>>) {
    if mouse.just_pressed(MouseButton::Left) {
        next_state.set(AppState::Game);
    }
}

fn cleanup(mut commands: Commands, tower: Query<Entity, With<TowerPlan>>) {
    commands
        .entity(tower.single())
        .remove::<TowerPlan>()
        .insert(Tower);
}
