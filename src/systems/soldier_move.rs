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

			log::info!("current_path : {}", soldier.current_path );

			let old_pos = TilePos {
				x: soldier.path[soldier.current_path - 1].0,
				y: soldier.path[soldier.current_path - 1].1,
			};

			let tile_origin_center = old_pos.center_in_world(grid_size, map_type).extend(1.0);
			let origin_trsf = *map_transform * Transform::from_translation(tile_origin_center);
			
			log::info!("origin_trsf.translation: {}/{}", origin_trsf.translation.x, origin_trsf.translation.y);


			let next_pos = TilePos {
				x: soldier.path[soldier.current_path].0,
				y: soldier.path[soldier.current_path].1
			};
			

			let tile_dest_center = next_pos.center_in_world(grid_size, map_type).extend(1.0);
			let dest_trsf = *map_transform * Transform::from_translation(tile_dest_center);
			
			
			log::info!("dest_trsf.translation: {}/{}", dest_trsf.translation.x, dest_trsf.translation.y);
			
			log::info!("time: {}", time.delta_seconds());

			let delta_x = (dest_trsf.translation.x - origin_trsf.translation.x) * time.delta_seconds();
			let delta_y = (dest_trsf.translation.y - origin_trsf.translation.y) * time.delta_seconds();	

			log::info!("delta: {delta_x}/{delta_y}");

			let mut soldier_transform = soldier_transform_q.single_mut();

			soldier_transform.translation.x += delta_x;
			soldier_transform.translation.y += delta_y;
			
			log::info!("soldier_transform: {}/{}", soldier_transform.translation.x, soldier_transform.translation.y);

			if soldier_transform.translation.x.floor() == dest_trsf.translation.x &&
				soldier_transform.translation.y.floor() == dest_trsf.translation.y {
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