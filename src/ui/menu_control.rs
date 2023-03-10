use bevy::{prelude::*, ui::PositionType};

use crate::{
    events::{CloseMenu, OpenMenu},
    world::{Game, GameFSM, UIFiniteStateMachine},
};

pub fn menu_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    ui_state: ResMut<UIFiniteStateMachine>,
    mut ev_open_main_menu: EventWriter<OpenMenu>,
    mut ev_close_main_menu: EventWriter<CloseMenu>,
) {
    match ui_state.status {
        GameFSM::Menu => {
            if keyboard_input.just_released(KeyCode::Escape) {
                match ui_state.menu_entity {
                    Some(_) => ev_close_main_menu.send(CloseMenu),
                    None => {}
                }
            }
        }
        GameFSM::Game => {
            if keyboard_input.just_released(KeyCode::Escape) {
                match ui_state.menu_entity {
                    Some(_) => {}
                    None => ev_open_main_menu.send(OpenMenu),
                }
            }
        }
    }
}
