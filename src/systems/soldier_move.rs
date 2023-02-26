use bevy::{prelude::*, log};
use bevy_ecs_tilemap::{prelude::{TilemapGridSize, TilemapType}, tiles::TilePos};
use bevy::time::Stopwatch;

use crate::components::soldier::{Soldier, SoldierPos};

#[derive(Component)]
pub struct MyTimer {
    pub time: Stopwatch,
}

pub fn soldier_move(
	time: Res<Time>,
	mut soldier_q: Query<&mut Soldier>,
	mut soldier_transform_q: Query<&mut Transform, With<Soldier>>,
	tilemap_q: Query<(
		&Transform,
        &TilemapGridSize,
		&TilemapType),  Without<Soldier>>,
) {
    if !soldier_q.is_empty()
		&& !soldier_transform_q.is_empty()
		&& !tilemap_q.is_empty()
	{
		let mut soldier = soldier_q.single_mut();

		if !soldier.move_done && soldier.path.len() > 1 {
			
			let (map_transform, grid_size, map_type) = tilemap_q.single();

			// Get origin
			let origin_pos = TilePos {
				x: soldier.path[soldier.current_tile].0,
				y: soldier.path[soldier.current_tile].1,
			};
			let tile_origin_center = origin_pos.center_in_world(grid_size, map_type).extend(1.0);
			let origin_trsf = *map_transform * Transform::from_translation(tile_origin_center);
			
			// Get destination
			let dest_pos = TilePos {
				x: soldier.path[soldier.current_tile + 1].0,
				y: soldier.path[soldier.current_tile + 1].1
			};
			let tile_dest_center = dest_pos.center_in_world(grid_size, map_type).extend(1.0);
			let dest_trsf = *map_transform * Transform::from_translation(tile_dest_center);
			
			// Get delta from timer
			let delta_x = (dest_trsf.translation.x - origin_trsf.translation.x) * time.delta_seconds();
			let delta_y = (dest_trsf.translation.y - origin_trsf.translation.y) * time.delta_seconds();	

			// Set the position
			let mut soldier_transform = soldier_transform_q.single_mut();
			soldier_transform.translation.x += delta_x;
			soldier_transform.translation.y += delta_y;
			
			// Go to next tile when destination reached
			if soldier_transform.translation.x.floor() == dest_trsf.translation.x &&
				soldier_transform.translation.y.floor() == dest_trsf.translation.y {
					log::info!("next tile");
					soldier.current_tile += 1;
			}

			// Stop all moves when all tiles covered
			if soldier.current_tile + 1 == soldier.path.len() {
				log::info!("move_done");
				soldier.move_done = true;
				soldier.current_pos = SoldierPos(dest_pos.x, dest_pos.y);
			}
		}
    }
}