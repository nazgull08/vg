use bevy::prelude::*;

use crate::{
    events::{CloseMenu, OpenMenu},
    world::{GameFSM, UIFiniteStateMachine},
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
                if let Some(_) = ui_state.menu_entity {
                    ev_close_main_menu.send(CloseMenu)
                }
            }
        }
        GameFSM::Game => {
            if keyboard_input.just_released(KeyCode::Escape) {
                if let None = ui_state.menu_entity {
                    ev_open_main_menu.send(OpenMenu)
                }
            }
        }
    }
}
