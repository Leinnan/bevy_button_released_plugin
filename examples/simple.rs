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
    **q_text.single_mut() = format!("Last button released: {}", button_name);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(root()).with_children(|parent| {
        parent.spawn((
            Text::new("Press any button below"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            node(50.0),
        ));
        for (text, color) in [("Green", GREEN), ("Red", RED), ("Yellow", YELLOW)] {
            parent
                .spawn((
                    Button,
                    node(80.0),
                    BackgroundColor(color.into()),
                    Name::new(text),
                ))
                .observe(on_button);
        }
    });
}

fn root() -> impl Bundle {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(50.0)),
        justify_content: JustifyContent::SpaceEvenly,
        flex_direction: FlexDirection::Column,
        ..default()
    }
}

fn node(height: f32) -> Node {
    Node {
        height: Val::Px(height),
        ..default()
    }
}
