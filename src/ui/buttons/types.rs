use bevy::prelude::Component;

#[derive(Component, PartialEq, Debug)]
pub enum Buttons {
    MainMenuButton(MainMenuButtons),
    OptionsButton(OptionsButtons),
}

#[derive(PartialEq, Debug)]
pub enum MainMenuButtons {
    NewGame,
    LoadGame,
    Options,
    Exit,
}

#[derive(PartialEq, Debug)]
pub enum OptionsButtons {
    Language,
    Option1,
    Option2,
}

#[derive(Component, PartialEq, Debug)]
pub struct ButtonTag {
    pub tag: Buttons,
}
