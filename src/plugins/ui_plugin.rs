use bevy::{prelude::*};

use crate::{systems::{ui_spawn::ui_spawn, button_system::button_system}, resources::game_state::GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState { started: false, map_loaded: false, ui_spawn: false })
            .add_system(ui_spawn);
            //.add_system(button_system);
    }
}
