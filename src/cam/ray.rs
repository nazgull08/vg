use bevy::prelude::*;
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

use crate::control::types::HoveredEntity;

pub fn cast_ray(
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut hovered: ResMut<HoveredEntity>,
) {
    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in cameras.iter() {
        // First, compute a ray from the mouse position.
        let (ray_pos, ray_dir) =
            ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);

        // Then cast the ray.
        let hit = rapier_context.cast_ray(ray_pos, ray_dir, f32::MAX, true, QueryFilter::new());

        if let Some((entity, _toi)) = hit {
            // Color in blue the entity we just hit.
            // Because of the query filter, only colliders attached to a dynamic body
            hovered.value = Some(entity);
            return;
        } else {
            hovered.value = None;
            return;
        }
    }
}

pub fn cast_ray_center(
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut hovered: ResMut<HoveredEntity>,
) {
    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in cameras.iter() {
        // First, compute a ray from the mouse position.
        let (ray_pos, ray_dir) =
            ray_from_center(windows.get_primary().unwrap(), camera, camera_transform);

        // Then cast the ray.
        let hit = rapier_context.cast_ray(ray_pos, ray_dir, f32::MAX, true, QueryFilter::new());

        if let Some((entity, _toi)) = hit {
            // Color in blue the entity we just hit.
            // Because of the query filter, only colliders attached to a dynamic body
            hovered.value = Some(entity);
            return;
        } else {
            hovered.value = None;
            return;
        }
    }
}

// Credit to @doomy on discord.
pub fn ray_from_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));

    let x = 2.0 * (mouse_position.x / window.width() as f32) - 1.0;
    let y = 2.0 * (mouse_position.y / window.height() as f32) - 1.0;

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}

// Credit to @doomy on discord.
pub fn ray_from_center(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let _mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));

    let x = 0.0;
    let y = 0.0;

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}
