use bevy::prelude::{Color, Commands, ResMut};
use bevy_rapier3d::render::ColliderDebugColor;

use super::types::{HoveredEntity, SelectedEntity};

pub fn hovered_entity_tracker(
    mut commands: Commands,
    mut hovered: ResMut<HoveredEntity>,
    selected: ResMut<SelectedEntity>,
) {
    match selected.value {
        Some(sel_ent) => match hovered.value {
            Some(ent) => {
                match hovered.last {
                    Some(ent_last) => {
                        if sel_ent != ent_last {
                            if ent != ent_last {
                                hovered.last = Some(ent);
                            }
                        }
                    }
                    None => {
                        hovered.last = Some(ent);
                        return;
                    }
                }
            }
            None => match hovered.last {
                Some(ent_last) => {
                    if ent_last != sel_ent {
                        let color = Color::RED;
                        commands.entity(ent_last).insert(ColliderDebugColor(color));
                    }
                    hovered.last = None;
                }
                None => {
                    return;
                }
            },
        },
        None => match hovered.value {
            Some(ent) => {
                let color = Color::BLUE;
                commands.entity(ent).insert(ColliderDebugColor(color));
                match hovered.last {
                    Some(ent_last) => {
                        if ent != ent_last {
                            let color = Color::RED;
                            commands.entity(ent_last).insert(ColliderDebugColor(color));
                            hovered.last = Some(ent);
                        }
                    }
                    None => {
                        hovered.last = Some(ent);
                        return;
                    }
                }
            }
            None => match hovered.last {
                Some(ent_last) => {
                    let color = Color::RED;
                    commands.entity(ent_last).insert(ColliderDebugColor(color));
                    hovered.last = None;
                }
                None => {
                    return;
                }
            },
        },
    }
}
