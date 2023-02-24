use bevy::{prelude::*, log};
use bevy_ecs_tilemap::{prelude::{TilemapGridSize, TilemapType}, tiles::TilePos};

use crate::components::soldier::{Soldier, SoldierPos};

pub fn soldier_move(
	mut soldier_q: Query<&mut Soldier>,
	mut soldier_transform_q: Query<&mut Transform, With<Soldier>>,
	tilemap_q: Query<(
		&Transform,
        &TilemapGridSize,
		&TilemapType),  Without<Soldier>>,
	time: Res<Time>,
) {
    if !soldier_q.is_empty()
		&& !soldier_transform_q.is_empty()
		&& !tilemap_q.is_empty()
	{

		let mut soldier = soldier_q.single_mut();

		if soldier.current_path == soldier.path.len() {
			soldier.move_done = true;
		}

		if !soldier.move_done {

			log::info!("current_path : {}", soldier.current_path );

			let next_pos = TilePos {
				x: soldier.path.get(soldier.current_path).unwrap().0,
				y: soldier.path[soldier.current_path].1
			};

			log::info!("next_pos : {}/{}", next_pos.x, next_pos.y );

			//log::info!("soldier_move");
			
			let (map_transform, grid_size, map_type) = tilemap_q.single();

			let mut soldier_transform = soldier_transform_q.single_mut();

			let tile_center = next_pos.center_in_world(grid_size, map_type).extend(1.0);

			log::info!("tile_center: {}/{}",tile_center.x, tile_center.y);


			let transform = *map_transform * Transform::from_translation(tile_center);

			
			
			let w_x = transform.translation.x;// * time.delta_seconds();
			let w_y = transform.translation.y;// * time.delta_seconds();
			
			log::info!("world_pos: {w_x}/{w_y}");

			soldier_transform.translation.x = w_x;
			soldier_transform.translation.y = w_y;
			
			soldier.move_done = true;

			soldier.current_pos = SoldierPos(next_pos.x, next_pos.y);

			//if w_x >= 1. || w_y >= 1. {
			//	soldier.current_path += 1;
			//}
			
		}
    }
}