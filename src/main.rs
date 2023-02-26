use bevy::prelude::*;

use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use voidgrinder::{
    cam::{
        orbit::{pan_orbit_camera, spawn_orbit_camera},
        ray::cast_ray,
    },
    control::{
        hover::hovered_entity_tracker,
        movement::move_selected,
        select::selected_entity_tracker,
        types::{HoveredEntity, SelectedEntity},
    },
    events::*,
    physics::{clean_forces, MoveCDTimer},
    ui::{buttons::button_system, main_menu::setup_main_menu},
    units::eyelegger::spawn_eye_legger,
    world::{ship::ship_startup, Game},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<SpawnBall>()
        .add_event::<SpawnEyeLegger>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_light)
        .add_startup_system(spawn_orbit_camera)
        .add_startup_system(setup_main_menu)
        .add_startup_system(ship_startup)
        .init_resource::<Game>()
        .insert_resource(MoveCDTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(HoveredEntity {
            value: None,
            last: None,
        })
        .insert_resource(SelectedEntity {
            value: None,
            last: None,
        })
        .add_system(clean_forces)
        .add_system(pan_orbit_camera)
        .add_system(cast_ray)
        .add_system(hovered_entity_tracker)
        .add_system(selected_entity_tracker)
        .add_system(button_system)
        .add_system(spawn_eye_legger)
        .add_system(move_selected)
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
            intensity: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(20.0, 5.0, 20.0),
        ..default()
    });
}
