use bevy::prelude::{Entity, Resource};

#[derive(Resource, Default)]
pub struct Game {
    pub players: Vec<Option<Entity>>,
    pub ground: Option<Entity>,
    pub score: i32,
    pub cake_eaten: u32,
}
