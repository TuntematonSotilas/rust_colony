use bevy::{prelude::*, log, math::Vec4Swizzles};
use bevy_ecs_tilemap::{tiles::{TilePos}, prelude::{TilemapSize, TilemapGridSize, TilemapType}};

pub fn mouse_click(
	windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
	camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
	tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
		&Transform,
    )>,
) {
    if buttons.just_released(MouseButton::Left) {

		let window = windows.get_primary().unwrap();

    	if let Some(position) = window.cursor_position() {
        	// cursor is inside the window, position given
			log::info!("Left Button was released : {position}");

			// get the camera info and transform
			// assuming there is exactly one main camera entity, so query::single() is OK
			let (camera, camera_transform) = camera_q.single();
			
			// check if the cursor is inside the window and get its position
			if let Some(screen_position) = window.cursor_position() {
				// check if it is possible to create a ray from screen coordinates to world coordinates
				if let Some(ray) = camera.viewport_to_world(camera_transform, screen_position) {
					// get 2d world mouse coordinates from the ray
					let world_position: Vec2 = ray.origin.truncate();
					
					log::info!("World coords: {world_position}");

					let (map_size, grid_size, map_type, map_transform) = tilemap_q.single();


					let cursor_pos = Vec4::from((world_position, 1.0, 1.0));

					let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
					let cursor_in_map_pos_xy = cursor_in_map_pos.xy();

					log::info!("cursor_in_map_pos_xy: {cursor_in_map_pos_xy}");


					// Once we have a world position we can transform it into a possible tile position.
					if let Some(tile_pos) = TilePos::from_world_pos(&cursor_in_map_pos_xy, map_size, grid_size, map_type)
					{
						let x = tile_pos.x;
						let y = tile_pos.y;
						log::info!("tile_pos: {x}/{y}");
					}
				}
			}
			
    	} 

    }
}
