use bevy::prelude::*;
use kayak_ui::{prelude::KayakContextPlugin, widgets::KayakWidgets, CameraUIKayak};

use crate::{resources::game_state::GameState, systems::ui_spawn::ui_spawn};

pub struct UiPlugin;

fn setup(mut commands: Commands) {
    // 2d camera
    commands
        .spawn(Camera2dBundle::default())
        .insert(CameraUIKayak);
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
           .add_plugin(KayakWidgets)
           .add_startup_system(setup)
           .insert_resource(GameState {
                started: false,
                map_loaded: false,
                ui_spawn: false,
            })
            .add_system(ui_spawn);
    }
}
