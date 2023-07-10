use bevy::prelude::*;

//Colors
pub const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HOVERED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
pub const PRESSED_BUTTON: Color = Color::rgb(0.1, 0.2, 0.1);

//Size
pub const STD_SIZE: (Val, Val) = (Val::Px(150.), Val::Px(70.));

pub const MENU_BG: Color = Color::Rgba {
    red: 0.05,
    green: 0.05,
    blue: 0.05,
    alpha: 1.0,
};

pub fn std_bttn() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Percent(40.0),
            height: Val::Percent(15.0),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(20.0)),
            // position
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

pub fn std_txt(text: &str, font: &Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font.to_owned(),
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    )
}
