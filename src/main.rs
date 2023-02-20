//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData, Collider, RapierContext, QueryFilter, RigidBody, Velocity, ExternalForce}, render::{RapierDebugRenderPlugin, ColliderDebugColor}, rapier::prelude::InteractionGroups};
use voidgrinder::camera::{pan_orbit_camera, spawn_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_light)
        .add_startup_system(setup_physics)
        .add_startup_system(spawn_camera)
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
        .add_system(pan_orbit_camera)
        .add_system(cast_ray)
        .add_system(camera_tracker_debug)
        .add_system(hovered_entity_tracker)
        .add_system(selected_entity_tracker)
        .add_system(move_selected)
        .run();
}

#[derive(Resource)]
struct MoveCDTimer(Timer);

#[derive(Resource)]
struct CameraTrackerTimer(Timer);

#[derive(Resource,Debug)]
struct HoveredEntity{ value: Option<Entity>, last: Option<Entity>}

#[derive(Resource,Debug)]
struct SelectedEntity{ value: Option<Entity>, last: Option<Entity>}

#[derive(Component,Debug)]
struct Selected{selection : bool}

fn camera_tracker_debug(
    time: Res<Time>,
    mut timer: ResMut<CameraTrackerTimer>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    hovered: ResMut<HoveredEntity>,
    selected: ResMut<SelectedEntity>,
    ){
    // make sure we wait enough time before spawning the next cake
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    for (camera, camera_transform) in cameras.iter() {
        let (ray_pos, ray_dir) =
                ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);
        info!("Camera:{:?}",ray_pos);
        info!("Selection status: {:?}",selected);
        info!("Hovering status: {:?}",hovered);
    }

}

fn selected_entity_tracker(
    mut commands: Commands,
    hovered: ResMut<HoveredEntity>,
    mut selected: ResMut<SelectedEntity>,
    mouse_button_input: ResMut<'_, Input<MouseButton>>,
    mut query: Query<&Selected>,
    ){
    if mouse_button_input.pressed(MouseButton::Left){
        selected.last = selected.value;
        selected.value = hovered.value;
        match selected.value {
            Some(ent) => {
                for (sel) in query.iter_mut() {
                    info!("{:?}",sel);
                }
                let color = Color::GREEN;
                commands.entity(ent).insert(Selected{selection: true});
                commands.entity(ent).insert(ColliderDebugColor(color));
                match selected.last {
                    Some(ent_last) => {
                    if ent != ent_last{
                        let color = Color::RED;
                        commands.entity(ent_last).insert(Selected{selection: false});
                        commands.entity(ent_last).insert(ColliderDebugColor(color));
                    }
                    },
                    None => {
                        return;
                    }
                }
            },
            None => {
                match selected.last {
                    Some(ent_last) => {
                        let color = Color::RED;
                        commands.entity(ent_last).insert(Selected{selection: false});
                        commands.entity(ent_last).insert(ColliderDebugColor(color));
                    },
                    None => {
                        return;
                    }
                }
            }
            
        }
    }
}
fn hovered_entity_tracker(
    mut commands: Commands,
    mut hovered: ResMut<HoveredEntity>,
    selected: ResMut<SelectedEntity>,
    ){
    match selected.value {
        Some(sel_ent) => {
            match hovered.value {
                Some(ent) => {
                    if sel_ent != ent {
                    let color = Color::BLUE;
                    commands.entity(ent).insert(ColliderDebugColor(color));
                    };
                    match hovered.last {
                        Some(ent_last) => {
                            if sel_ent != ent_last{
                                if ent != ent_last{
                                    let color = Color::RED;
                                    commands.entity(ent_last).insert(ColliderDebugColor(color));
                                    hovered.last = Some(ent);
                                }
                            }
                        },
                        None => {
                            hovered.last = Some(ent);
                            return;
                        }

                    }
                },
                None => {
                    match hovered.last{
                        Some(ent_last) => {
                            if ent_last != sel_ent {
                                let color = Color::RED;
                                commands.entity(ent_last).insert(ColliderDebugColor(color));
                            }
                            hovered.last = None;
                        },
                        None => {
                            return;
                        }
                    }
                }
            }
           
        },
        None => {
            match hovered.value {
                Some(ent) => {
                    let color = Color::BLUE;
                    commands.entity(ent).insert(ColliderDebugColor(color));
                    match hovered.last {
                        Some(ent_last) => {
                            if ent != ent_last{
                            let color = Color::RED;
                            commands.entity(ent_last).insert(ColliderDebugColor(color));
                            hovered.last = Some(ent);
                            }
                        },
                        None => {
                            hovered.last = Some(ent);
                            return;
                        }

                    }
                },
                None => {
                    match hovered.last{
                        Some(ent_last) => {
                            let color = Color::RED;
                            commands.entity(ent_last).insert(ColliderDebugColor(color));
                            hovered.last = None;
                        },
                        None => {
                            return;
                        }
                    }
                }
            }
        }
    }

}

/// set up a simple 3D scene
fn setup_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    let rad = 1.0;


    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(rad, rad, rad),
    )).insert(ExternalForce {
        force: Vec3::new(0.0, 0.0, 0.0),
        torque: Vec3::new(0.0, 0.0, 0.0),
    }).insert(Selected{selection: false});

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 5.0, 3.0)),
        RigidBody::Dynamic,
        Collider::cuboid(rad, rad, rad),
    )).insert(ExternalForce {
        force: Vec3::new(0.0, 0.0, 0.0),
        torque: Vec3::new(0.0, 0.0, 0.0),
    }).insert(Selected{selection: false});
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(-5.0, 5.0, 3.0)),
        RigidBody::Dynamic,
        Collider::ball(rad),
    )).insert(ExternalForce {
        force: Vec3::new(0.0, 0.0, 0.0),
        torque: Vec3::new(0.0, 0.0, 0.0),
    }).insert(Selected{selection: false});
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(10.0, 5.0, 3.0)),
        RigidBody::Dynamic,
        Collider::cylinder(rad, rad),
    )).insert(ExternalForce {
        force: Vec3::new(0.0, 0.0, 0.0),
        torque: Vec3::new(0.0, 0.0, 0.0),
    }).insert(Selected{selection: false});
    
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
    mut query: Query<(&mut ExternalForce, &Selected)>,
) {
        match selected.value {
            Some(ent) => {
                let mut moved = false;
                let mut way = (0.0,0.0);
                if keyboard_input.pressed(KeyCode::Up) {
                    way=(1.0,0.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    way=(-1.0,0.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    way=(0.0,1.0);
                    moved = true;
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    way=(0.0,-1.0);
                    moved = true;
                }
        if moved {
               for (mut f_e,sel) in query.iter_mut() {
                  if sel.selection {
                   info!("{:?} Force: {:?}",way,f_e);
                    f_e.force = Vec3::new(50.0*way.0, 0.0, 50.0*way.1);
                  } 
                } 
        }


            }
            None => {
                return;
            } 

    }
}
