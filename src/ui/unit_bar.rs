use bevy::prelude::*;

use crate::{
    events::{CloseUnitBar, OpenUnitBar},
    world::{GameFSM, UIFiniteStateMachine},
};

use super::buttons::{
    rules::{std_bttn, std_txt, MENU_BG},
    types::{ButtonTag, Buttons, MainMenuButtons},
};

pub fn unit_bar_setup(
    mut commands: Commands,
    mut ev_spawn_main_menu: EventWriter<OpenUnitBar>,
    mut ui_state: ResMut<UIFiniteStateMachine>,
) {
    ev_spawn_main_menu.send(OpenUnitBar);
}

pub fn unit_bar_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_spawn_unit_bar: EventReader<OpenUnitBar>,
    mut ev_close_unit_bar: EventReader<CloseUnitBar>,
    mut ui_state: ResMut<UIFiniteStateMachine>,
) {
    for _ in ev_spawn_unit_bar.iter() {
        let font: Handle<Font> = asset_server.load("fonts/TrigramLight-w1XDz.ttf");

        ui_state.unit_bar_entity = Some(
            commands
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Auto,
                            ..default()
                        },
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Center,
                            margin: UiRect {
                                top: Val::Auto,
                                ..default()
                            },
                            width: Val::Percent(100.),
                            height: Val::Percent(20.),
                            ..default()
                        },
                        background_color: MENU_BG.into(),
                        ..default()
                    });
                })
                .id(),
        );
    }
    for _ in ev_close_unit_bar.iter() {
        if let Some(me) = ui_state.menu_entity {
            commands.entity(me).despawn_recursive();
            ui_state.menu_entity = None;
            ui_state.status = GameFSM::Game;
        }
    }
}
