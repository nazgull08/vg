use bevy::prelude::*;

use crate::world::Game;

const RESET_FOCUS: [f32; 3] = [15.0, 0.0, 30. / 2.0 - 0.5];

pub fn spawn_focus_camera(mut commands: Commands, mut game: ResMut<Game>) {
    game.camera_should_focus = Vec3::from(RESET_FOCUS);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            -(30 as f32 / 2.0),
            2.0 * 30 as f32 / 3.0,
            30 as f32 / 2.0 - 0.5,
        )
        .looking_at(game.camera_is_focus, Vec3::Y),
        ..default()
    });
}

// change the focus of the camera
pub fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    const SPEED: f32 = 2.0;
    if game.players.len() > 0 {
        // if there is both a player and a bonus, target the mid-point of them
        if let Some(player_entity) = game.players[0] {
            if let Ok(player_transform) = transforms.p1().get(player_entity) {
                game.camera_should_focus = player_transform.translation;
            }
            // otherwise, target the middle
        } else {
            game.camera_should_focus = Vec3::from(RESET_FOCUS);
        }
    } else {
        game.camera_should_focus = Vec3::from(RESET_FOCUS);
    }
    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        game.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for mut transform in transforms.p0().iter_mut() {
        *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
    }
}
