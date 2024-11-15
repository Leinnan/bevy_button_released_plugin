# Bevy Button Released plugin

[![crates.io](https://img.shields.io/crates/v/bevy_button_released_plugin.svg)](https://crates.io/crates/bevy_button_released_plugin)
[![license](https://img.shields.io/crates/l/bevy_button_released_plugin)](https://github.com/Leinnan/bevy_button_released_plugin#license)
[![crates.io](https://img.shields.io/crates/d/bevy_button_released_plugin.svg)](https://crates.io/crates/bevy_button_released_plugin)

This crate makes [Bevy](https://github.com/bevyengine/bevy) application aware of the release of the button instead of reacting right after clicking. I think it will be addressed in next release but until then it could be helpful for some people.

# Install

```
cargo add bevy_button_released_plugin
```

# Usage

Add `ButtonsReleasedPlugin` during app creation process, then the `GameButton` component will be added to buttons and it is possible to react to it like in `button_system` function in example below. Auto adding `GameButton` can be disabled by disabling default "auto_add" feature.

```rust
use bevy::{color::palettes::css::*, prelude::*};
use bevy_button_released_plugin::{ButtonsReleasedPlugin, OnButtonReleased};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ButtonsReleasedPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn on_button(trigger: Trigger<OnButtonReleased>, q: Query<&Name>, mut q_text: Query<&mut Text>) {
    let mut text = q_text.single_mut();
    let Ok(button_name) = q.get(trigger.entity()) else {
        return;
    };
    text.sections[0].value = format!("Last button released: {}", button_name);
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // root node
    commands.spawn(root()).with_children(|parent| {
        parent.spawn(top_text());
        for (text, color) in [("Green", GREEN), ("Red", RED), ("Yellow", YELLOW)] {
            parent
                .spawn((
                    ButtonBundle {
                        style: style(),
                        background_color: BackgroundColor(color.into()),
                        ..default()
                    },
                    Name::new(text),
                ))
                .observe(on_button);
        }
    });
}
fn top_text() -> TextBundle {
    TextBundle::from_section("Press any button below", default())
        .with_text_justify(JustifyText::Center)
        .with_style(style())
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceEvenly,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }
}

fn style() -> Style {
    let rect = UiRect::all(Val::Px(30.0));
    Style {
        margin: rect,
        padding: rect,
        ..default()
    }
}
```

# Bevy compatibility table

| Bevy version | Crate version |
| ------------ | ------------- |
| 0.14         | 0.6, 0.7      |
| 0.13         | 0.5           |
| 0.12         | 0.3,0.4       |
| 0.11         | 0.2           |
| 0.10         | 0.1           |
