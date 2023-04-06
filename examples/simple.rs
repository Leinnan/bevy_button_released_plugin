use bevy::prelude::*;
use bevy_button_click_plugin::{ButtonReleasedEvent, ButtonsReleasedPlugin, GameButton};

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(ButtonsReleasedPlugin)
        .add_startup_system(setup)
        .add_system(button_system);
    app.run();
}

fn button_system(mut reader: EventReader<ButtonReleasedEvent>, q: Query<&Name>) {
    for event in reader.iter() {
        if let Ok(button_name) = q.get(**event) {
            info!("Button released: {}", button_name);
        }
    }
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for (text, color) in [
                ("GreenButton", Color::GREEN),
                ("RedButton", Color::ORANGE_RED),
                ("YellowButton", Color::YELLOW),
            ] {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(18.0)),
                            padding: UiRect::all(Val::Px(30.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(color),
                        ..default()
                    },
                    GameButton::default(),
                    Name::new(text),
                ));
            }
        });
}
