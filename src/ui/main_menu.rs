use bevy::prelude::*;

use crate::{
    events::{CloseMenu, OpenMenu},
    world::{GameFSM, UIFiniteStateMachine},
};

use super::buttons::{
    rules::{std_bttn, std_txt, MENU_BG},
    types::{ButtonTag, Buttons, MainMenuButtons},
};

pub fn main_menu_setup(mut ev_spawn_main_menu: EventWriter<OpenMenu>) {
    ev_spawn_main_menu.send(OpenMenu);
}

pub fn main_menu_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_spawn_main_menu: EventReader<OpenMenu>,
    mut ev_close_main_menu: EventReader<CloseMenu>,
    mut ui_state: ResMut<UIFiniteStateMachine>,
) {
    for _ in ev_spawn_main_menu.iter() {
        // ui camera
        //let font:Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
        let font: Handle<Font> = asset_server.load("fonts/TrigramLight-w1XDz.ttf");

        ui_state.status = GameFSM::Menu;

        ui_state.menu_entity = Some(
            commands
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            ..default()
                        },
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                        },
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|child| {
                    child
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    ..default()
                                },
                                size: Size {
                                    width: Val::Percent(30.),
                                    height: Val::Percent(60.),
                                },
                                ..default()
                            },
                            background_color: MENU_BG.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(std_bttn())
                                .with_children(|parent| {
                                    parent.spawn(std_txt("Новая игра", &font));
                                })
                                .insert(ButtonTag {
                                    tag: Buttons::MainMenuButton(MainMenuButtons::NewGame),
                                });
                            parent
                                .spawn(std_bttn())
                                .with_children(|parent| {
                                    parent.spawn(std_txt("Загрузить игру", &font));
                                })
                                .insert(ButtonTag {
                                    tag: Buttons::MainMenuButton(MainMenuButtons::LoadGame),
                                });
                            parent
                                .spawn(std_bttn())
                                .with_children(|parent| {
                                    parent.spawn(std_txt("Опции", &font));
                                })
                                .insert(ButtonTag {
                                    tag: Buttons::MainMenuButton(MainMenuButtons::Options),
                                });
                            parent
                                .spawn(std_bttn())
                                .with_children(|parent| {
                                    parent.spawn(std_txt("Выход", &font));
                                })
                                .insert(ButtonTag {
                                    tag: Buttons::MainMenuButton(MainMenuButtons::Exit),
                                });
                        });
                })
                .id(),
        )
    }
    for _ in ev_close_main_menu.iter() {
        match ui_state.menu_entity {
            Some(me) => {
                commands.entity(me).despawn_recursive();
                ui_state.menu_entity = None;
                ui_state.status = GameFSM::Game;
            }
            None => {}
        }
    }
}
