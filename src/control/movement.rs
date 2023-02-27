use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use crate::{physics::MoveCDTimer, world::Game};

use super::types::{Selected, SelectedEntity};

pub fn move_selected(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    selected: ResMut<SelectedEntity>,
    _transforms: Query<&mut Transform>,
    _time: Res<Time>,
    _timer: ResMut<MoveCDTimer>,
    game_state: ResMut<Game>,
    query: Query<&Selected>,
) {
    match selected.value {
        Some(_ent) => {
            let mut moved = false;
            let mut way = Vec3::new(0.0, 0.0, 0.0);
            if keyboard_input.pressed(KeyCode::Up) {
                way = Vec3::new(1.0, 0.0, 0.0);
                moved = true;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                way = Vec3::new(-1.0, 0.0, 0.0);
                moved = true;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                way = Vec3::new(0.0, 0.0, 1.0);
                moved = true;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                way = Vec3::new(0.0, 0.0, -1.0);
                moved = true;
            }
            if keyboard_input.pressed(KeyCode::LControl) {
                way = Vec3::new(0.0, 1.0, 0.0);
                moved = true;
            }
            if keyboard_input.pressed(KeyCode::Back) {
                way = Vec3::new(0.0, -1.0, 0.0);
                moved = true;
            }
            if moved {
                for splayer in &game_state.players {
                    match splayer {
                        None => {}
                        Some(player) => {
                            if let Ok(sel) = query.get(player.to_owned()) {
                                if sel.selection == true {
                                    commands.entity(player.to_owned()).insert(ExternalForce {
                                        force: (way * 70.0),
                                        torque: (way * 0.0),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            return;
        }
    }
}
