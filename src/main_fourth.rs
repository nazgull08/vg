//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData, Collider, RapierContext, QueryFilter, RigidBody, Velocity, ExternalForce, Damping, Restitution, CoefficientCombineRule}, render::RapierDebugRenderPlugin};
use voidgrinder::{camera::{pan_orbit_camera, spawn_camera}, control::{types::{HoveredEntity, SelectedEntity, Selected}, hover::hovered_entity_tracker, select::selected_entity_tracker}, ui::{stats::stats_setup, button::{setup_button, button_system}}, world::Game, events::*, units::ball::spawn_ball, player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<SpawnBall>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_light)
        .add_startup_system(setup_physics)
        .add_startup_system(spawn_camera)
        .add_startup_system(stats_setup)
        .add_startup_system(setup_button)
        .init_resource::<Game>()
        .insert_resource(CameraTrackerTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(MoveCDTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(HoveredEntity{value: None,last: None})
        .insert_resource(SelectedEntity{value: None,last: None})
        .add_system(clean_forces)
        .add_system(pan_orbit_camera)
        .add_system(cast_ray)
        .add_system(hovered_entity_tracker)
        .add_system(selected_entity_tracker)
        .add_system(move_selected)
        .add_system(button_system)
        .add_system(spawn_ball)
        .run();
}

#[derive(Resource)]
struct MoveCDTimer(Timer);

#[derive(Resource)]
struct CameraTrackerTimer(Timer);



/// set up a simple 3D scene
fn setup_light(
    mut commands: Commands,
) {
    // plane
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(20.0, 15.0, 4.0),
        ..default()
    });
}

fn clean_forces(
    time: Res<Time>,
    mut timer: ResMut<MoveCDTimer>,
    mut ext_forces: Query<&mut ExternalForce>,
    game_state: ResMut<Game>
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    for splayer in &game_state.players{
        match splayer {
            None => {},
            Some(player) => {
                if let Ok(mut ext_force) = ext_forces.get_mut(player.to_owned()){
                    ext_force.force = Vec3::new(0.0,0.0,0.0);
                    ext_force.torque = Vec3::new(0.0,0.0,0.0);
                }
            }
        }
    }
}

pub fn setup_physics(mut commands: Commands,
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

    game_state.ground = Some(commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box{max_x:gmax_x
            ,min_x:gmin_x
            ,min_y:gmin_y
            ,max_y:gmax_y
            ,min_z:gmin_z
            ,max_z:gmax_z})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(ground_size*1.05, ground_height*1.05, ground_size*1.05))
    .insert(Restitution {
        coefficient: 0.7,
        combine_rule: CoefficientCombineRule::Min,
    })
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 2.0))).id());

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
        base_color: Color::Rgba { red: 0.4, green: 0.0, blue: 0.8, alpha: 0.9 },
        alpha_mode: AlphaMode::Blend,
        metallic: 0.9,
        ..default()
    });

    players.push(Some(commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{ size: rad*2.0})),
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
    .insert(Damping { linear_damping: dld, angular_damping: dad })
    .insert(Restitution {
        coefficient: 0.7,
        combine_rule: CoefficientCombineRule::Min,
    })
    .insert(Selected{selection: false}).id()));

    players.push(Some(commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{ size: rad*2.0})),
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
    .insert(Damping { linear_damping: dld, angular_damping: dad })
    .insert(Selected{selection: false}).id()));

    players.push(Some(commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere{ radius: rad, subdivisions: 8})),
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
    .insert(Damping { linear_damping: dld, angular_damping: dad })
    .insert(Restitution {
        coefficient: 0.7,
        combine_rule: CoefficientCombineRule::Min,
    })
    .insert(Selected{selection: false}).id()));

    players.push(Some(commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere{ radius: rad, subdivisions: 8})),
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
    .insert(Damping{ linear_damping: dld, angular_damping: dad })
    .insert(Selected{selection: false}).id()));

    game_state.players = players;
}


fn cast_ray(
    mut commands: Commands,
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
        let hit = rapier_context.cast_ray(
            ray_pos,
            ray_dir,
            f32::MAX,
            true,
            QueryFilter::only_dynamic(),
        );

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
fn ray_from_mouse_position(
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

fn move_selected(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut selected: ResMut<SelectedEntity>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    mut timer: ResMut<MoveCDTimer>,
    mut game_state: ResMut<Game>,
    query: Query<&Selected>
) {
        match selected.value {
            Some(ent) => {
                let mut moved = false;
                let mut way = Vec3::new(0.0,0.0,0.0); 
                if keyboard_input.pressed(KeyCode::Up) {
                    way=Vec3::new(1.0,0.0,0.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    way=Vec3::new(-1.0,0.0,0.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    way=Vec3::new(0.0,0.0,1.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    way=Vec3::new(0.0,0.0,-1.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Space) {
                    way=Vec3::new(0.0,1.0,0.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Back) {
                    way=Vec3::new(0.0,-1.0,0.0);
                    moved = true;
                }
        if moved {
        
            for splayer in &game_state.players{
                match splayer {
                    None => {},
                    Some(player) => {
                        if let Ok(sel) = query.get(player.to_owned()) {
                            if sel.selection == true {
                                commands.entity(player.to_owned()).insert(ExternalForce {
                                    force: (way*50.0), 
                                    torque: (way*0.0),

                                });
                            }
                        }
                    }
                    
                }
            }
        
        }


            }
            None => {
                return;
            } 

    }
}
