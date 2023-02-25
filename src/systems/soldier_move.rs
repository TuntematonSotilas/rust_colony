use bevy::{prelude::*, log};
use bevy_ecs_tilemap::{prelude::{TilemapGridSize, TilemapType}, tiles::TilePos};

use crate::components::soldier::{Soldier, SoldierPos};

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

			log::info!("current_path : {}", soldier.current_path );

			let next_pos = TilePos {
				x: soldier.path.get(soldier.current_path).unwrap().0,
				y: soldier.path[soldier.current_path].1
			};
			

			let tile_dest_center = next_pos.center_in_world(grid_size, map_type).extend(1.0);
			let dest_trsf = *map_transform * Transform::from_translation(tile_dest_center);
			
			log::info!("dest_trsf.translation: {}/{}", dest_trsf.translation.x, dest_trsf.translation.y);
			
			let delta_x = dest_trsf.translation.x * time.delta_seconds() * 2.;
			let delta_y = dest_trsf.translation.y * time.delta_seconds() * 2.;			
			log::info!("delta: {delta_x}/{delta_y}");

			let mut soldier_transform = soldier_transform_q.single_mut();

			soldier_transform.translation.x += delta_x;
			soldier_transform.translation.y += delta_y;
			
			log::info!("soldier_transform: {}/{}", soldier_transform.translation.x, soldier_transform.translation.y);

			if ((delta_x >= 0. && soldier_transform.translation.x >= dest_trsf.translation.x) ||
					(delta_x < 0. && soldier_transform.translation.x <= dest_trsf.translation.x)) &&
			   ((delta_y >= 0. && soldier_transform.translation.y >= dest_trsf.translation.y) ||
					(delta_y < 0. && soldier_transform.translation.y <= dest_trsf.translation.y)) {
					log::info!("next move");
					soldier.current_path += 1;
			}

			if soldier.current_path == soldier.path.len() {
				log::info!("move_done");
				soldier.move_done = true;
				soldier.current_pos = SoldierPos(next_pos.x, next_pos.y);

			}
			
		}
    }
}