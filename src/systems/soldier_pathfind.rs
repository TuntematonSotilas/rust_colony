use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TilePos,
};
use pathfinding::prelude::bfs;

use crate::{
    components::soldier::{Pos, Soldier},
    resources::cursor_state::CursorState,
    utils::{constant::Z_MAP_BASE_LAYER, position::tile_to_world},
};

#[allow(clippy::needless_pass_by_value)]
pub fn soldier_pathfind(
    mut cursor_state: ResMut<CursorState>,
    tilemap_q: Query<(&Transform, &TilemapSize, &TilemapGridSize, &TilemapType), Without<Soldier>>,
    mut soldier_sprite_q: Query<(&mut Soldier, &mut Transform)>,
) {
    if !cursor_state.pathfind_done && !tilemap_q.is_empty() && !soldier_sprite_q.is_empty() {
        cursor_state.pathfind_done = true;

        if let Some(tile_pos) = cursor_state.click_pos {
            for (map_transform, map_size, grid_size, map_type) in &tilemap_q {
                if map_transform.translation.z == Z_MAP_BASE_LAYER {
                    let mut solider_pos = TilePos { x: 10, y: 10 };
                    let (mut soldier, mut soldier_trsf) = soldier_sprite_q.single_mut();

                    if let Some(init_pos) = soldier.init_pos {
                        solider_pos = init_pos;
                        soldier.init_pos = None;
                    } else {
                        // Get soldier tile
                        let sol_pos = Vec4::from((soldier.current_pos, 1.0, 1.0));
                        let sol_in_map_pos = map_transform.compute_matrix().inverse() * sol_pos;
                        let sol_in_map_pos_xy = sol_in_map_pos.xy();

                        if let Some(pos) = TilePos::from_world_pos(
                            &sol_in_map_pos_xy,
                            map_size,
                            grid_size,
                            map_type,
                        ) {
                            solider_pos = pos;
                            //re-center the soldier on the tile
                            soldier_trsf.translation =
                                tile_to_world(pos, *grid_size, *map_type, map_transform);
                        }
                    }

                    // Pathfinding
                    let goal = Pos(tile_pos.x, tile_pos.y);
                    let current_pos = Pos(solider_pos.x, solider_pos.y);

                    if current_pos != goal {
                        #[allow(clippy::redundant_closure_for_method_calls)]
                        let result = bfs(&current_pos, |p| p.successors(), |p| *p == goal);
                        if let Some(result) = result {
                            soldier.path = result;
                            soldier.move_done = false;
                            soldier.current_tile = 0;
                            soldier.dir_set = false;
                        }
                    }
                }
            }
        }
    }
}
