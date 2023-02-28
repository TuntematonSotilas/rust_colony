use bevy::{log, math::Vec4Swizzles, prelude::*};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TilePos,
};

use pathfinding::prelude::bfs;

use crate::components::soldier::{Pos, Soldier};

#[allow(clippy::needless_pass_by_value)]
pub fn mouse_click(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform), Without<Soldier>>,
    mut soldier_transform_q: Query<&mut Transform, With<Soldier>>,
    mut soldier: Query<&mut Soldier>,
) {
    if buttons.just_released(MouseButton::Left)
        && !camera_q.is_empty()
        && !tilemap_q.is_empty()
        && !soldier.is_empty()
		&& !soldier_transform_q.is_empty()
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
                if let Some(tile_pos) =
                    TilePos::from_world_pos(&cursor_in_map_pos_xy, map_size, grid_size, map_type)
                {
                    
					let mut solider_pos: TilePos = TilePos { x: 10, y: 10 };
					if let Some(init_pos) = soldier.init_pos {
						log::info!("init");
						solider_pos = init_pos;
						soldier.init_pos = None;
					} else {
						
						// Get soldier tile 
						let sol_pos = Vec4::from((soldier.current_pos, 1.0, 1.0));
						let sol_in_map_pos = map_transform.compute_matrix().inverse() * sol_pos;
						let sol_in_map_pos_xy = sol_in_map_pos.xy();

						if let Some(pos) = TilePos::from_world_pos(&sol_in_map_pos_xy, map_size, grid_size, map_type)
						{
							solider_pos = pos;

							log::info!("current_pos {}/{}", pos.x, pos.y);

							//re-center the soldier on the tile
							let tile_center = pos.center_in_world(grid_size, map_type).extend(1.0);
        					let transform = *map_transform * Transform::from_translation(tile_center);

							let mut soldier_transform = soldier_transform_q.single_mut();
							soldier_transform.translation = transform.translation;
							
						}
					}
					
					let goal = Pos(tile_pos.x, tile_pos.y);

                    log::info!("goal: {}/{}", goal.0, goal.1);
                    //log::info!("soldier.current_pos: {}/{}", soldier.current_pos.0, soldier.current_pos.1);


					let pos = Pos(solider_pos.x, solider_pos.y);
					let result = bfs(&pos, |p| p.successors(), |p| *p == goal);
					if let Some(result) = result {
						soldier.path = result.clone();
						soldier.move_done = false;
						soldier.current_tile = 0;
					}
                }
            }
        }
    }
}
