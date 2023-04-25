use bevy::prelude::*;

use crate::{resources::game_state::GameState, systems::ui_spawn::ui_spawn};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState {
            started: false,
            map_loaded: false,
            ui_spawn: false,
        })
        .add_system(ui_spawn);
    }
}
