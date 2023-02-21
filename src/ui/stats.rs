use bevy::{prelude::{Commands, AssetServer, Res, TextBundle, Color}, text::TextStyle, ui::Style, utils::default};
use bevy::ui::*;



pub fn stats_setup(mut commands: Commands,
    asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "Score:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
    );
}
