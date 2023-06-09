use bevy::{
    prelude::{App, Changed, Component, Deref, Entity, EventWriter, Plugin, Query},
    ui::Interaction,
};

pub struct ButtonsReleasedPlugin;

#[derive(Deref)]
pub struct ButtonReleasedEvent(Entity);

#[derive(Component, Default)]
pub struct GameButton {
    pub last_state: Interaction,
}

impl Plugin for ButtonsReleasedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_click_system)
            .add_event::<ButtonReleasedEvent>();
    }
}

fn button_click_system(
    mut interaction_query: Query<(Entity, &Interaction, &mut GameButton), Changed<Interaction>>,
    mut ev: EventWriter<ButtonReleasedEvent>,
) {
    for (entity, interaction, mut game_button) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if game_button.last_state == Interaction::Clicked {
                    ev.send(ButtonReleasedEvent(entity));
                }
            }
            _ => {}
        }
        game_button.last_state = *interaction;
    }
}
