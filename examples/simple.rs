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
    let button_name = q.get(trigger.entity()).expect("Missing Name!");
    q_text.single_mut().sections[0].value = format!("Last button released: {}", button_name);
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
