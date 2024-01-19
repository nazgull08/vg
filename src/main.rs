<<<<<<< HEAD
use std::f32::consts::PI;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::DirectionalLightShadowMap,
    prelude::*,
};

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
    ui::{
        buttons::systems::{button_system, main_menu_button_system},
        main_menu::{main_menu_setup, main_menu_system},
        menu_control::menu_control_system,
        unit_bar::{unit_bar_setup, unit_bar_system},
    },
    units::{ball::spawn_ball, eyelegger::spawn_eye_legger},
    world::{ship::ship_startup, Game, UIFiniteStateMachine},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CloseMenu>()
        .add_event::<OpenMenu>()
        .add_event::<OpenUnitBar>()
        .add_event::<CloseUnitBar>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(main_menu_setup)
        .add_startup_system(setup_directional_light)
        .add_startup_system(spawn_orbit_camera)
        .add_startup_system(ship_startup)
        .init_resource::<Game>()
        .init_resource::<UIFiniteStateMachine>()
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
        .add_system(menu_control_system)
        .add_system(unit_bar_system)
        .add_system(main_menu_system)
        .add_system(pan_orbit_camera)
        .add_system(cast_ray)
        .add_system(hovered_entity_tracker)
        .add_system(selected_entity_tracker)
        .add_system(button_system)
        .add_system(main_menu_button_system)
        .add_system(spawn_ball)
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

/// set up a simple 3D scene
fn setup_directional_light(mut commands: Commands) {
    // plane
    // light
    const HALF_SIZE: f32 = 500.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            color: Color::BLUE,
            // Configure the projection to better fit the scene
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 50.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}
=======
>>>>>>> c98859257774433af1f306e574edcee48e6c2fbb
