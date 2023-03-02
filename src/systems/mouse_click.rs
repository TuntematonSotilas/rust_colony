use bevy::{math::Vec4Swizzles, prelude::*, log};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TilePos,
};

use pathfinding::prelude::bfs;

use crate::{components::soldier::{Pos, Soldier}, utils::pos_util::tile_to_world};

#[allow(clippy::needless_pass_by_value)]
pub fn mouse_click(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform), Without<Soldier>>,
    mut soldier_sprite_q: Query<(&mut Transform, &mut TextureAtlasSprite), With<Soldier>>,
    mut soldier_q: Query<&mut Soldier>,
	texture_atlases: Res<Assets<TextureAtlas>>,
) {
    if buttons.just_released(MouseButton::Left)
        && !camera_q.is_empty()
        && !tilemap_q.is_empty()
        && !soldier_sprite_q.is_empty()
		&& !soldier_q.is_empty()
		&& !texture_atlases.is_empty()
    {
        let window = windows.get_primary().unwrap();

        let mut soldier = soldier_q.single_mut();

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
					let (mut soldier_trsf, mut soldier_sprite)  = soldier_sprite_q.single_mut();

					if let Some(init_pos) = soldier.init_pos {
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
							//re-center the soldier on the tile
							soldier_trsf.translation = tile_to_world(pos, grid_size, map_type, map_transform);
						}
					}

					// Pathfinding
					let goal = Pos(tile_pos.x, tile_pos.y);
					let pos = Pos(solider_pos.x, solider_pos.y);
					let result = bfs(&pos, |p| p.successors(), |p| *p == goal);
					if let Some(result) = result {
						soldier.path = result.clone();
						soldier.move_done = false;
						soldier.current_tile = 0;
					}
					
					// Set direction
					if goal.0 > pos.0 {
						soldier_sprite.index = 1;
						log::info!("right");
					}
					
                }
            }
        }
    }
}
