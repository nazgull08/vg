use bevy::ecs::event::Event;

#[derive(Event)]
pub struct SpawnBall;
#[derive(Event)]
pub struct SpawnEyeLegger;

#[derive(Event)]
pub struct OpenMenu;
#[derive(Event)]
pub struct CloseMenu;

#[derive(Event)]
pub struct OpenUnitBar;
#[derive(Event)]
pub struct CloseUnitBar;
