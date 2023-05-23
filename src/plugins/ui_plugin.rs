use bevy::prelude::*;
use kayak_ui::{prelude::KayakContextPlugin, widgets::KayakWidgets, CameraUIKayak};

use crate::{systems::ui_spawn::ui_spawn, resources::player_state::{PlayerState, PlayerRace}};

pub struct UiPlugin;

fn setup(mut commands: Commands) {
    // 2d camera
    commands
        .spawn(Camera2dBundle::default())
        .insert(CameraUIKayak);
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState { player_race: PlayerRace::Human })
            .add_plugin(KayakContextPlugin)
			.add_plugin(KayakWidgets)
			.add_startup_system(setup)
			.add_system(ui_spawn);
	}
}
