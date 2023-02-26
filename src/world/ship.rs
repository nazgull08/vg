use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{CoefficientCombineRule, Collider, Restitution, RigidBody},
    render::ColliderDebugColor,
};

use crate::control::types::Selected;

use super::{Cell, Game, Vec3i32};

use rand::prelude::*;

const SHIP_X: i32 = 30;
const SHIP_Y: i32 = 30;
const SHIP_Z: i32 = 1;

pub fn ship_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("res/img/cobble1.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        ..default()
    });

    let ground_size = 200.1;
    let ground_height = 0.1;
    let gmin_x = 0.0 - ground_size;
    let gmax_x = 0.0 + ground_size;
    let gmin_z = 0.0 - ground_size;
    let gmax_z = 0.0 + ground_size;
    let gmin_y = 0.0 - ground_height;
    let gmax_y = 0.0 + ground_height;

    let _dld = 10.0;
    let _dad = 10.0;

    game.ground = Some(
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
            .insert(Collider::cuboid(
                ground_size * 1.05,
                ground_height * 1.05,
                ground_size * 1.05,
            ))
            .insert(TransformBundle::from(Transform::from_xyz(
                0.0,
                -ground_height - 2.0,
                2.0,
            )))
            .id(),
    );

    game.map.cells = (0..SHIP_X)
        .map(|i| {
            (0..SHIP_Y)
                .map(|j| {
                    (0..SHIP_Z)
                        .map(|k| {
                            let mut rng = rand::thread_rng();
                            let rwall: f64 = rng.gen();
                            if (k < 1)
                                || ((k < 2) && (rwall > 0.5))
                                || ((k < 3) && (rwall > 0.7))
                                || ((k >= 3) && (rwall > 0.9))
                            {
                                info!("k: {k}, rw: {rwall}");
                                game.players.push(Some(
                                    commands
                                        .spawn(PbrBundle {
                                            mesh: meshes.add(Mesh::from(shape::Box {
                                                max_x: 1.,
                                                min_x: -1.,
                                                min_y: -1.,
                                                max_y: 1.,
                                                min_z: -1.,
                                                max_z: 1.,
                                            })),
                                            material: material_handle.clone(),
                                            ..default()
                                        })
                                        .insert(Collider::cuboid(1., 1., 1.))
                                        .insert(RigidBody::Dynamic)
                                        .insert(ColliderDebugColor(Color::RED))
                                        .insert(Restitution {
                                            coefficient: 0.7,
                                            combine_rule: CoefficientCombineRule::Min,
                                        })
                                        .insert(Selected { selection: false })
                                        .insert(TransformBundle::from(Transform::from_xyz(
                                            2.0 * (i as f32),
                                            4.0 * (k as f32),
                                            2.0 * (j as f32),
                                        )))
                                        .id(),
                                ));
                            }
                            Cell {
                                position: Vec3i32::new(i, j, k),
                                object: None,
                            }
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
}
