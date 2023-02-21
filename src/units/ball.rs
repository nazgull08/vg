use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody, ExternalForce, Velocity, Damping};

use crate::{world::Game, control::types::Selected, events::SpawnBall};



pub fn spawn_ball(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<Game>,
    mut ev_spawn_ball: EventReader<SpawnBall>,
    ) {

    for ev_sp_ball in ev_spawn_ball.iter(){
        let mut players = vec![];
        let dad = 1.0;
        let dld = 1.0;

        let rad = 1.0;

        // load a texture and retrieve its aspect ratio
        let texture_handle = asset_server.load("res/img/cat.png");
        let aspect = 0.25;

    // this material renders the texture normally
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        players.push(Some(commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere{ radius: rad, subdivisions: 8})),
            material: material_handle,
            ..default()
        })
        .insert(Collider::ball(rad))
        .insert(TransformBundle::from(Transform::from_xyz(-5.0, 10.0, 3.0)))
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0., 0.0, 0.0),
        })
        .insert(Damping{ linear_damping: dld, angular_damping: dad })
        .insert(Selected{selection: false}).id()));

        game_state.players = players;

    }
}
