use bevy::{app::AppExit, prelude::*};

use crate::{
    events::{CloseMenu, OpenUnitBar, SpawnBall, SpawnEyeLegger},
    ui::buttons::{
        rules::PRESSED_BUTTON,
        types::{Buttons, MainMenuButtons},
    },
};

use super::{
    rules::{HOVERED_BUTTON, NORMAL_BUTTON},
    types::ButtonTag,
};

pub fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                // position
                position_type: PositionType::Absolute,
                top: Val::Px(250.0),
                right: Val::Px(300.0),
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button3",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                // position
                position_type: PositionType::Absolute,
                top: Val::Px(400.0),
                right: Val::Px(5.0),
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button2",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    _ev_spawn_eye_legger: EventWriter<SpawnEyeLegger>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn main_menu_button_system(
    mut interaction_query: Query<(&Interaction, &ButtonTag), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
    mut ev_spawn: EventWriter<SpawnBall>,
    mut ev_close_menu: EventWriter<CloseMenu>,
    mut ev_open_unit_bar: EventWriter<OpenUnitBar>,
) {
    for (interaction, bt_tag) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Button: {:?}", bt_tag);
                match &bt_tag.tag {
                    Buttons::MainMenuButton(m_m_b) => match m_m_b {
                        MainMenuButtons::NewGame => {
                            ev_spawn.send(SpawnBall);
                            ev_close_menu.send(CloseMenu);
                            ev_open_unit_bar.send(OpenUnitBar);
                        }
                        MainMenuButtons::LoadGame => {}
                        MainMenuButtons::Options => {}
                        MainMenuButtons::Exit => {
                            exit.send(AppExit);
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
