# Bevy Button Released plugin

[![crates.io](https://img.shields.io/crates/v/bevy_button_released_plugin.svg)](https://crates.io/crates/bevy_button_released_plugin)
[![license](https://img.shields.io/crates/l/bevy_button_released_plugin)](https://github.com/NiklasEi/bevy_button_released_plugin#license)
[![crates.io](https://img.shields.io/crates/d/bevy_button_released_plugin.svg)](https://crates.io/crates/bevy_button_released_plugin)

This crate makes [Bevy](https://github.com/bevyengine/bevy) application aware of the release of the button instead of reacting right after clicking. I think it will be addressed in next release but until then it could be helpful for some people.

# Install

```
cargo add bevy_button_released_plugin
```

# Usage

Add `ButtonsReleasedPlugin` during app creation process, `GameButton` component to buttons that you want to react to it like in `button_system` function in example below.

```rust
use bevy::prelude::*;
use bevy_button_released_plugin::{ButtonReleasedEvent, ButtonsReleasedPlugin, GameButton};

#[derive(Component)]
pub struct Info;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(ButtonsReleasedPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system);
    app.run();
}

fn button_system(mut reader: EventReader<ButtonReleasedEvent>, q: Query<&Name>, mut q_text: Query<&mut Text,With<Info>>) {
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
            parent.spawn((
            TextBundle::from_section(
                "",
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
            Info,
        ));
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
```

# Bevy compatibility table
Bevy version | Crate version
--- | ---
0.12 | 0.3.0
0.11 | 0.2.0
0.10 | 0.1.0