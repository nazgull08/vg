use bevy::{prelude::*, ui::PositionType};

use super::buttons::rules::{NORMAL_BUTTON, STD_SIZE};

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(STD_SIZE.0, STD_SIZE.1),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                // position
                position_type: PositionType::Relative,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Новая игра",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
