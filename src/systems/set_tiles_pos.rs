use bevy::{prelude::*};
use bevy_ecs_tilemap::tiles::{TileStorage, TilePos};

use crate::resources::tiles_pos::{TilesPos, Pos};

pub fn set_tiles_pos(
	tilemap_q: Query<&TileStorage>,
	tile_q: Query<&mut TilePos>,
	mut tiles_pos: ResMut<TilesPos>,
) {
	let tiles_pos = tiles_pos.as_mut();

	if !tiles_pos.set && !tilemap_q.is_empty()
	{
		tiles_pos.pos = Vec::new();

		let tilemap_storage = tilemap_q.single();

		for tile_entity in tilemap_storage.iter().flatten() {
			let tile_pos = tile_q.get(*tile_entity).unwrap();
			 let x = tile_pos.x as i32;
			 let y = tile_pos.y as i32;
			 tiles_pos.pos.push(Pos(x,y));
		 }
			
		tiles_pos.set = true;
	}
	

}
