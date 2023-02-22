use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{
        CoefficientCombineRule, Collider, Damping, ExternalForce, NoUserData, QueryFilter,
        RapierContext, RapierPhysicsPlugin, Restitution, RigidBody, Velocity,
    },
    render::RapierDebugRenderPlugin,
};
use voidgrinder::{
    camera::{cast_ray, pan_orbit_camera, spawn_camera},
    control::{
        hover::hovered_entity_tracker,
        select::selected_entity_tracker,
        types::{HoveredEntity, Selected, SelectedEntity},
    },
    events::*,
    ui::{
        button::{button_system, setup_button},
        stats::stats_setup,
    },
    world::Game,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<SpawnBall>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_light)
        .add_startup_system(spawn_camera)
        .add_startup_system(stats_setup)
        .add_startup_system(setup_button)
        .init_resource::<Game>()
        .insert_resource(HoveredEntity {
            value: None,
            last: None,
        })
        .insert_resource(SelectedEntity {
            value: None,
            last: None,
        })
        .add_system(pan_orbit_camera)
        .add_system(cast_ray)
        .add_system(hovered_entity_tracker)
        .add_system(selected_entity_tracker)
        .add_system(button_system)
        .run();
}

#[derive(Resource)]
struct CameraTrackerTimer(Timer);

/// set up a simple 3D scene
fn setup_light(mut commands: Commands) {
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
}
