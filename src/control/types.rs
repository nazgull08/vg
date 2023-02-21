use bevy::prelude::{Resource, Entity, Component};


#[derive(Resource,Debug)]
pub struct HoveredEntity{
    pub value: Option<Entity>,
    pub last: Option<Entity>}

#[derive(Resource,Debug)]
pub struct SelectedEntity{ 
    pub value: Option<Entity>, 
    pub last: Option<Entity>}

#[derive(Component,Debug)]
pub struct Selected{
    pub selection : bool}
