use bevy::{ui::{PositionType, UiRect}, prelude::*};


use bevy::{prelude::*, render::camera::Camera};
use bevy::ecs::system::IntoSystem;

struct MenuButton {
    text: String,
    active: bool,
}

pub fn setup_menu_test_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Создаем три кнопки
    let button1 = MenuButton {
        text: "Button 1".to_string(),
        active: false,
    };

    let button2 = MenuButton {
        text: "Button 2".to_string(),
        active: false,
    };

    let button3 = MenuButton {
        text: "Button 3".to_string(),
        active: false,
    };


    // Создаем кнопки
    let mut button_entities = Vec::new();

    for button in vec![button1, button2, button3].into_iter() {
        let button_entity = commands.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(20.0),
                    bottom: Val::Px(0.0),
                },
                ..Default::default()
            },
            //material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            ..Default::default()
        })
        .id();

        // Добавляем текст на кнопку
        commands.entity(button_entity).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Новая игра",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));

        });

        button_entities.push(button_entity);
    }

    // Размещаем кнопки на экране
    let window_width = 800.0;
    let window_height = 600.0;

    let mut y = window_height / 2.0 - 100.0;

    for button_entity in button_entities {
    }
}

