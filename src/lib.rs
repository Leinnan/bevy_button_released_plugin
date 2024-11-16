use bevy::ecs::system::Res;
#[cfg(feature = "auto_add")]
use bevy::ecs::*;
use bevy::input::touch::Touches;
use bevy::reflect::Reflect;
use bevy::{
    prelude::{App, Changed, Component, Entity, Event, Plugin, Query, Update},
    ui::*,
};
use system::Commands;

/// Plugin that adds `OnButtonReleased` event for entities with `GameButton` and `Interaction`
/// components when Interaction changes from `Interaction::Pressed`
/// to `Interaction::Hovered`.
pub struct ButtonsReleasedPlugin;

#[cfg(feature = "global_event")]
#[derive(bevy::prelude::Deref, Event, Reflect)]
pub struct ButtonReleasedEvent(Entity);

/// Event triggered when Interaction changes from `Interaction::Pressed`
/// to `Interaction::Hovered` for entity that has GameButton component.
/// It is using the observer pattern for events.
#[derive(Event, Reflect)]
pub struct OnButtonReleased;

/// Component storing previous interaction state.
/// When it is part of entity that also contains `Interaction` component
/// it will allow triggering `OnButtonReleased` when Interaction changes from `Interaction::Pressed`
/// to `Interaction::Hovered`.
#[derive(Component, Default, Reflect)]
#[require(Interaction)]
pub struct GameButton {
    pub last_state: Interaction,
}

impl Plugin for ButtonsReleasedPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "global_event")]
        app.add_event::<ButtonReleasedEvent>();
        app.register_type::<GameButton>()
            .register_type::<OnButtonReleased>()
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
    touches: Res<Touches>,
    mut interaction_query: Query<(Entity, &Interaction, &mut GameButton), Changed<Interaction>>,
    mut commands: Commands,
    #[cfg(feature = "global_event")] mut ev: event::EventWriter<ButtonReleasedEvent>,
) {
    let any_input_released = touches.any_just_released();
    for (entity, interaction, mut game_button) in &mut interaction_query {
        let was_hovered = any_input_released || *interaction == Interaction::Hovered;
        if was_hovered && game_button.last_state == Interaction::Pressed {
            commands.trigger_targets(OnButtonReleased, entity);
            #[cfg(feature = "global_event")]
            ev.send(ButtonReleasedEvent(entity));
        }

        game_button.last_state = *interaction;
    }
}
