use bevy::{prelude::*};

use crate::systems::set_tiles_pos::set_tiles_pos;
use crate::{systems::mouse_click::mouse_click, resources::tiles_pos::TilesPos};
use crate::systems::soldier_sys::spawn_soliders;

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
        app.insert_resource(TilesPos { pos: Vec::new(), set: false })
			.add_startup_system(setup)
			.add_startup_system(spawn_soliders)
			.add_system(set_tiles_pos)
			.add_system(mouse_click);
    }
}

