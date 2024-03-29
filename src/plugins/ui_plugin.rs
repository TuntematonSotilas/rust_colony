use bevy::prelude::*;
use kayak_ui::{prelude::KayakContextPlugin, widgets::KayakWidgets, CameraUIKayak};

use crate::{
    resources::player_state::{PlayerMap, PlayerRace, PlayerState},
    systems::ui_spawn::ui_spawn,
};

pub struct UiPlugin;

fn setup(mut commands: Commands) {
    // 2d camera
    commands
        .spawn(Camera2dBundle::default())
        .insert(CameraUIKayak);
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState {
            player_race: PlayerRace::Human,
            player_map: PlayerMap::Desert,
        })
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(setup)
        .add_system(ui_spawn);
    }
}
