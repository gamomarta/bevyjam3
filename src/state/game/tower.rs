use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::assets::Sprites;
use crate::state::AppState;

pub(super) struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tower.in_schedule(OnEnter(AppState::Game)))
            .add_system(show_radius.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Tower;

const TOWER_SPRITE_SCALE: f32 = 0.5;

#[derive(Component, Deref, DerefMut)]
pub struct ShootTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ShootRadius(f32);

const DEFAULT_SHOOT_RADIUS: f32 = 400.0;

#[derive(Component)]
struct ShootRadiusImage;

const SHOOT_RADIUS_COLOR: Color = Color::Rgba {
    red: 0.3,
    green: 0.0,
    blue: 0.8,
    alpha: 0.6,
};

fn spawn_tower(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteBundle {
            texture: sprites.tower.clone(),
            transform: Transform::from_translation(Vec3::new(-100.0, 250.0, 0.0))
                .with_scale(Vec3::splat(TOWER_SPRITE_SCALE)),
            ..Default::default()
        })
        .insert(ShootTimer(Timer::from_seconds(0.6, TimerMode::Once)))
        .insert(ShootRadius(DEFAULT_SHOOT_RADIUS))
        .insert(Tower)
        .with_children(|tower| {
            tower
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(DEFAULT_SHOOT_RADIUS / TOWER_SPRITE_SCALE).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(SHOOT_RADIUS_COLOR)),
                    transform: Transform::from_translation(2.0 * Vec3::Z),
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(ShootRadiusImage);
        });
}

fn show_radius(
    towers: Query<(&Transform, &Children), With<Tower>>,
    mut tower_radii: Query<&mut Visibility, With<ShootRadiusImage>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let Some(cursor_position) = window.cursor_position()
        .and_then(|viewport_position| camera.viewport_to_world(camera_transform, viewport_position))
        .map(|ray| ray.origin.truncate()) else { return; };

    for (tower_transform, children) in towers.iter() {
        let tower_position = tower_transform.translation.truncate();
        let distance = (cursor_position - tower_position).length();
        for &child in children {
            let Ok(mut radius_visibility) = tower_radii.get_mut(child) else { continue; };
            *radius_visibility = if distance <= 65.0
            /* TODO: magic number */
            {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
