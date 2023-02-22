use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, Damping, ExternalForce, Restitution, RigidBody, Velocity,
};

use crate::{control::types::Selected, world::Game};

#[derive(Resource)]
pub struct MoveCDTimer(Timer);

pub fn clean_forces(
    time: Res<Time>,
    mut timer: ResMut<MoveCDTimer>,
    mut ext_forces: Query<&mut ExternalForce>,
    game_state: ResMut<Game>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    for splayer in &game_state.players {
        match splayer {
            None => {}
            Some(player) => {
                if let Ok(mut ext_force) = ext_forces.get_mut(player.to_owned()) {
                    ext_force.force = Vec3::new(0.0, 0.0, 0.0);
                    ext_force.torque = Vec3::new(0.0, 0.0, 0.0);
                }
            }
        }
    }
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<Game>,
) {
    let mut players = vec![];

    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;
    let gmin_x = 0.0 - ground_size;
    let gmax_x = 0.0 + ground_size;
    let gmin_z = 0.0 - ground_size;
    let gmax_z = 0.0 + ground_size;
    let gmin_y = 0.0 - ground_height;
    let gmax_y = 0.0 + ground_height;

    let dld = 0.1;
    let dad = 0.1;

    game_state.ground = Some(
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    max_x: gmax_x,
                    min_x: gmin_x,
                    min_y: gmin_y,
                    max_y: gmax_y,
                    min_z: gmin_z,
                    max_z: gmax_z,
                })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..default()
            })
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::cuboid(
                ground_size * 1.05,
                ground_height * 1.05,
                ground_size * 1.05,
            ))
            .insert(Restitution {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(TransformBundle::from(Transform::from_xyz(
                0.0,
                -ground_height,
                2.0,
            )))
            .id(),
    );

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

    let mat_purple_glass = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.4,
            green: 0.0,
            blue: 0.8,
            alpha: 0.9,
        },
        alpha_mode: AlphaMode::Blend,
        metallic: 0.9,
        ..default()
    });

    players.push(Some(
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: rad * 2.0 })),
                material: material_handle.clone(),
                ..default()
            })
            .insert(Collider::cuboid(rad, rad, rad))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
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
            .insert(Restitution {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Selected { selection: false })
            .id(),
    ));

    players.push(Some(
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: rad * 2.0 })),
                material: mat_purple_glass,
                ..default()
            })
            .insert(Collider::cuboid(rad, rad, rad))
            .insert(TransformBundle::from(Transform::from_xyz(-4.0, dld, 0.0)))
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

    players.push(Some(
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: rad,
                    subdivisions: 8,
                })),
                material: material_handle,
                ..default()
            })
            .insert(Collider::ball(rad))
            .insert(TransformBundle::from(Transform::from_xyz(-dld, dld, 3.0)))
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
            .insert(Restitution {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Selected { selection: false })
            .id(),
    ));

    players.push(Some(
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: rad,
                    subdivisions: 8,
                })),
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                ..default()
            })
            .insert(Collider::ball(rad))
            .insert(TransformBundle::from(Transform::from_xyz(10.0, dld, 3.0)))
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

    game_state.players = players;
}
