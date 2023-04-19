use bevy::{prelude::*};
use crate::resources::game_state::GameState;

pub fn button_system(
    mut game_state: ResMut<GameState>,
    interaction_query: Query<&Interaction, With<Button>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            game_state.started = true;
        }
    }
}