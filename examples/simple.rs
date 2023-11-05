use bevy::prelude::*;
use bevy_button_released_plugin::{ButtonReleasedEvent, ButtonsReleasedPlugin, GameButton};

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(ButtonsReleasedPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system);
    app.run();
}

fn button_system(
    mut reader: EventReader<ButtonReleasedEvent>,
    q: Query<&Name>,
    mut q_text: Query<&mut Text>,
) {
    let mut text = q_text.single_mut();
    for event in reader.read() {
        if let Ok(button_name) = q.get(**event) {
            text.sections[0].value = format!("Last button released: {}", button_name);
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
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Press any button below",
                    TextStyle {
                        font_size: 30.0,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    margin: UiRect::all(Val::Px(18.0)),
                    padding: UiRect::all(Val::Px(30.0)),
                    ..default()
                }),
            );
            for (text, color) in [
                ("Green", Color::GREEN),
                ("Red", Color::ORANGE_RED),
                ("Yellow", Color::YELLOW),
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
