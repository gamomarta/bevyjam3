use bevy::prelude::*;

use crate::state::game::shooting::ShootRadiusImage;
use crate::state::AppState;
use crate::utils::cursor_coordinates_to_world_coordinates;

pub(super) struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_radius.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Tower;

fn show_radius(
    towers: Query<(&Transform, &Children), With<Tower>>,
    mut tower_radii: Query<&mut Visibility, With<ShootRadiusImage>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window.single();
    let camera = camera.single();

    let Some(cursor_position) = cursor_coordinates_to_world_coordinates(window, camera) else { return; };

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
