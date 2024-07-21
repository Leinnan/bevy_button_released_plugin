use bevy::prelude::*;
use bevy_button_released_plugin::{ButtonReleasedEvent, ButtonsReleasedPlugin};

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
            let style: Style = Style {
                margin: UiRect::all(Val::Px(18.0)),
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            };
            parent.spawn(
                TextBundle::from_section(
                    "Press any button below",
                    TextStyle {
                        font_size: 30.0,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(style.clone()),
            );
            for (text, color) in [
                ("Green", Color::LinearRgba(LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 })),
                ("Red", Color::LinearRgba(LinearRgba{red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0})),
                ("Yellow", Color::LinearRgba(LinearRgba{red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0})),
            ] {
                parent.spawn((
                    ButtonBundle {
                        style: style.clone(),
                        background_color: BackgroundColor(color),
                        ..default()
                    },
                    Name::new(text),
                ));
            }
        });
}
