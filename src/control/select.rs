use bevy::prelude::{Commands, ResMut, Input, MouseButton, Query, Color};
use bevy_rapier3d::render::ColliderDebugColor;

use super::types::{HoveredEntity, SelectedEntity, Selected};


pub fn selected_entity_tracker(
    mut commands: Commands,
    hovered: ResMut<HoveredEntity>,
    mut selected: ResMut<SelectedEntity>,
    mouse_button_input: ResMut<'_, Input<MouseButton>>,
    mut query: Query<&Selected>,
    ){
    if mouse_button_input.pressed(MouseButton::Left){
        selected.last = selected.value;
        selected.value = hovered.value;
        match selected.value {
            Some(ent) => {
                let color = Color::GREEN;
                commands.entity(ent).insert(Selected{selection: true});
                commands.entity(ent).insert(ColliderDebugColor(color));
                match selected.last {
                    Some(ent_last) => {
                    if ent != ent_last{
                        let color = Color::RED;
                        commands.entity(ent_last).insert(Selected{selection: false});
                        commands.entity(ent_last).insert(ColliderDebugColor(color));
                    }
                    },
                    None => {
                        return;
                    }
                }
            },
            None => {
                match selected.last {
                    Some(ent_last) => {
                        let color = Color::RED;
                        commands.entity(ent_last).insert(Selected{selection: false});
                        commands.entity(ent_last).insert(ColliderDebugColor(color));
                    },
                    None => {
                        return;
                    }
                }
            }
            
        }
    }
}
