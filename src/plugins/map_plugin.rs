use bevy::{prelude::*};

use crate::resources::soldiers_state::SoldiersState;
use crate::systems::{
	mouse_click::mouse_click,
	soldier_spawn::soldier_spawn,
	soldier_move::soldier_move,
};

use super::tiled_plugin::{TiledMap, TiledMapBundle};

pub struct MapPlugin;

#[allow(clippy::needless_pass_by_value)]
fn setup(mut commands: Commands, 
	asset_server: Res<AssetServer>,) {

    let map_handle: Handle<TiledMap> = asset_server.load("/public/maps/map.tmx");

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });

}


impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
			.insert_resource(SoldiersState { spawn_done: false })
			.add_startup_system(setup)
			.add_system(soldier_spawn)
			.add_system(soldier_move)
			.add_system(mouse_click);
    }
}

