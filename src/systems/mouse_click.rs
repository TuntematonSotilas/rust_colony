use bevy::{prelude::*, log, math::Vec4Swizzles};
use bevy_ecs_tilemap::{tiles::{TilePos}, prelude::{TilemapSize, TilemapGridSize, TilemapType}};

use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

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
) {
    if buttons.just_released(MouseButton::Left) {

		let window = windows.get_primary().unwrap();

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
					let x = tile_pos.x as i32;
					let y = tile_pos.y as i32;
					log::info!("goal: {x}/{y}");

					let goal: Pos = Pos(x, y);
					let result = bfs(&Pos(0, 0), |p| p.successors(), |p| *p == goal);
					if let Some(result) = result
					{
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

impl Pos {
	fn successors(&self) -> Vec<Pos> {
	  let &Pos(x, y) = self;
	  vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
		   Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
	}
  }