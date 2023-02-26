use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_flycam::{PlayerPlugin, MovementSettings};
use voidgrinder::{
    control::{
        hover::hovered_entity_tracker,
        select::selected_entity_tracker,
        types::{HoveredEntity, SelectedEntity}, movement::move_selected,
    },
    events::*,
    ui::{
        button::{button_system, setup_button},
        stats::stats_setup, main_menu::setup_main_menu, main_menu_test::setup_menu_test_system,
    },
    world::{Game, ship::ship_startup}, units::{ball::spawn_ball, eyelegger::spawn_eye_legger}, physics::{MoveCDTimer, clean_forces}, cam::{focus::{spawn_focus_camera, focus_camera}, orbit::{pan_orbit_camera, spawn_orbit_camera}, ray::{cast_ray, cast_ray_center}},
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<SpawnBall>()
        .add_event::<SpawnEyeLegger>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_light)
        .add_startup_system(spawn_orbit_camera)
        .add_startup_system(stats_setup)
        .add_startup_system(setup_button)
        .add_startup_system(setup_main_menu)
        .add_startup_system(ship_startup)
        .init_resource::<Game>()
        .insert_resource(MoveCDTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
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

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 5.0, 10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(30.0, 5.0, 35.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 5.0, 35.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 30000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 30000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 45.0),
        ..default()
    });
}
