use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Damping, ExternalForce, RigidBody, Velocity};

use crate::{control::types::Selected, events::SpawnEyeLegger, world::Game};

pub fn spawn_eye_legger(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<Game>,
    mut ev_spawn_eye_legger: EventReader<SpawnEyeLegger>,
) {
    for _ev_sp_eye_legger in ev_spawn_eye_legger.iter() {
        let dad = 1.0;
        let dld = 1.0;

        let rad = 1.0;

        // load a texture and retrieve its aspect ratio
        //let _texture_handle = asset_server.load("res/img/cat.png");
        // load a texture and retrieve its aspect ratio
        // let model_handle = asset_server.load("models/AlienCake/alien.glb#Scene0");
        //info!("model: {:?} ", model_handle);

        // this material renders the texture normally

        game_state.players.push(Some(
            commands
                .spawn(SceneBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, 20.0, 0.0),
                        ..default()
                    },
                    scene: asset_server.load("models/eyelegger.glb#Scene0"),
                    ..default()
                })
                .insert(Collider::cuboid(rad * 4.0, rad * 4.0, rad * 4.0))
                .insert(TransformBundle::from(Transform::from_xyz(10.0, 30.0, 5.0)))
                .insert(RigidBody::Dynamic)
                .insert(ExternalForce {
                    force: Vec3::new(0.0, 0.0, 0.0),
                    torque: Vec3::new(0.0, 0.0, 0.0),
                })
                .insert(Velocity {
                    linvel: Vec3::new(0.0, 0.0, 0.0),
                    angvel: Vec3::new(0., 0.0, 0.0),
                })
                .insert(Damping {
                    linear_damping: dld,
                    angular_damping: dad,
                })
                .insert(Selected { selection: false })
                .id(),
        ));
    }
}
