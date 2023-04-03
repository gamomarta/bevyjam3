use bevy::prelude::*;

pub fn cursor_coordinates_to_world_coordinates(
    window: &Window,
    camera: (&Camera, &GlobalTransform),
) -> Option<Vec2> {
    let (camera, camera_transform) = camera;

    window
        .cursor_position()
        .and_then(|viewport_position| camera.viewport_to_world(camera_transform, viewport_position))
        .map(|ray| ray.origin.truncate())
}
