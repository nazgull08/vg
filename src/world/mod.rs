pub mod ship;

use bevy::prelude::{Entity, Resource, Vec3};

#[derive(Resource, Default)]
pub struct UIFiniteStateMachine {
    pub status: GameFSM,
    pub menu_entity: Option<Entity>,
    pub unit_bar_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub enum GameFSM {
    #[default]
    Menu,
    Game,
}

#[derive(Resource, Default)]
pub struct Game {
    pub players: Vec<Option<Entity>>,
    pub ground: Option<Entity>,
    pub map: WorldMap,
    pub score: i32,
    pub cake_eaten: u32,
    pub camera_should_focus: Vec3,
    pub camera_is_focus: Vec3,
}

#[derive(Resource, Default)]
pub struct WorldMap {
    pub cells: Vec<Vec<Vec<Cell>>>,
}

#[derive(Resource, Default)]
pub struct Cell {
    pub position: Vec3i32,
    pub object: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct Vec3i32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3i32 {
    fn new(x: i32, y: i32, z: i32) -> Vec3i32 {
        Vec3i32 { x, y, z }
    }
}
