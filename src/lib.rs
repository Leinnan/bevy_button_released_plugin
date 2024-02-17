#[cfg(feature = "auto_add")]
use bevy::ecs::*;
use bevy::reflect::Reflect;
use bevy::{
    prelude::{App, Changed, Component, Deref, Entity, Event, EventWriter, Plugin, Query, Update},
    ui::*,
};

pub struct ButtonsReleasedPlugin;

#[derive(Deref, Event, Reflect)]
pub struct ButtonReleasedEvent(Entity);

#[derive(Component, Default, Reflect)]
pub struct GameButton {
    pub last_state: Interaction,
}

impl Plugin for ButtonsReleasedPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonReleasedEvent>()
            .register_type::<GameButton>()
            .register_type::<ButtonReleasedEvent>()
            .add_systems(Update, button_click_system);
        #[cfg(feature = "auto_add")]
        app.add_systems(bevy::app::PostUpdate, add_game_button);
    }
}

#[cfg(feature = "auto_add")]
fn add_game_button(
    q: Query<Entity, (query::With<widget::Button>, query::Without<GameButton>)>,
    mut commands: system::Commands,
) {
    for e in q.iter() {
        commands.entity(e).insert(GameButton::default());
    }
}

fn button_click_system(
    mut interaction_query: Query<(Entity, &Interaction, &mut GameButton), Changed<Interaction>>,
    mut ev: EventWriter<ButtonReleasedEvent>,
) {
    for (entity, interaction, mut game_button) in &mut interaction_query {
        if *interaction == Interaction::Hovered && game_button.last_state == Interaction::Pressed {
            ev.send(ButtonReleasedEvent(entity));
        }

        game_button.last_state = *interaction;
    }
}
