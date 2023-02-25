use bevy::{prelude::*, log, math::Vec4Swizzles};
use bevy_ecs_tilemap::{tiles::{TilePos}, prelude::{TilemapSize, TilemapGridSize, TilemapType}};

use pathfinding::prelude::bfs;

use crate::components::soldier::{Soldier, SoldierPos};

pub fn mouse_click(
	windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
	camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
	tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
		&Transform
    )>,
	mut soldier: Query<&mut Soldier>,
) {
    if buttons.just_released(MouseButton::Left) 
		&& !camera_q.is_empty() 
		&& !tilemap_q.is_empty()
		&& !soldier.is_empty()
		{

		let window = windows.get_primary().unwrap();
		
		let mut soldier = soldier.single_mut();
		
		// get the camera info and transform
		// assuming there is exactly one main camera entity, so query::single() is OK
		let (camera, camera_transform) = camera_q.single();
		
		let (map_size, grid_size, map_type, map_transform) = tilemap_q.single();

		// check if the cursor is inside the window and get its position
		if let Some(screen_position) = window.cursor_position() {
			// check if it is possible to create a ray from screen coordinates to world coordinates
			if let Some(ray) = camera.viewport_to_world(camera_transform, screen_position) {
				// get 2d world mouse coordinates from the ray
				let world_position: Vec2 = ray.origin.truncate();
				let cursor_pos = Vec4::from((world_position, 1.0, 1.0));
				let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
				let cursor_in_map_pos_xy = cursor_in_map_pos.xy();

				// Once we have a world position we can transform it into a possible tile position.
				if let Some(tile_pos) = TilePos::from_world_pos(&cursor_in_map_pos_xy, map_size, grid_size, map_type)
				{

					let goal = SoldierPos(tile_pos.x, tile_pos.y);

					log::info!("goal: {}/{}", goal.0, goal.1);
					//log::info!("soldier.current_pos: {}/{}", soldier.current_pos.0, soldier.current_pos.1);

					let result = bfs(&soldier.current_pos, |p| p.successors(), |p| *p == goal);
					if let Some(result) = result
					{
						soldier.path = result.clone();
						soldier.move_done = false;
						soldier.current_path = 1;

						for r in result {
							let x = r.0;
							let y = r.1;
							log::info!("path: {x}/{y}");
						}
					}
				}
			}
		}

    }
}

