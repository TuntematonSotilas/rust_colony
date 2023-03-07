use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapType},
    tiles::TilePos,
};

use crate::{components::soldier::Soldier, utils::position::tile_to_world};

const SPEED: f32 = 2.;
const ERROR_MARGIN: f32 = 2.;

#[derive(Component)]
pub struct MyTimer {
    pub time: Stopwatch,
}

#[allow(clippy::needless_pass_by_value)]
pub fn soldier_move(
    time: Res<Time>,
    mut soldier_q: Query<&mut Soldier>,
    mut soldier_trsf_q: Query<&mut Transform, With<Soldier>>,
    tilemap_q: Query<(&Transform, &TilemapGridSize, &TilemapType), Without<Soldier>>,
) {
    if !soldier_q.is_empty() && !soldier_trsf_q.is_empty() && !tilemap_q.is_empty() {
        let mut soldier = soldier_q.single_mut();

        if !soldier.move_done && soldier.path.len() > 1 {
            let mut soldier_trsf = soldier_trsf_q.single_mut();

            let (map_transform, grid_size, map_type) = tilemap_q.single();

            // Get origin
            let origin_tile = TilePos {
                x: soldier.path[soldier.current_tile].0,
                y: soldier.path[soldier.current_tile].1,
            };
            let origin = tile_to_world(origin_tile, *grid_size, *map_type, map_transform);
            // Get destination
            let dest_tile = TilePos {
                x: soldier.path[soldier.current_tile + 1].0,
                y: soldier.path[soldier.current_tile + 1].1,
            };
            let dest = tile_to_world(dest_tile, *grid_size, *map_type, map_transform);

            // Get delta from timer
            let delta_x = (dest.x - origin.x) * time.delta_seconds() * SPEED;
            let delta_y = (dest.y - origin.y) * time.delta_seconds() * SPEED;

            // Set the position
            soldier_trsf.translation.x += delta_x;
            soldier_trsf.translation.y += delta_y;
            soldier.current_pos = Vec2::new(soldier_trsf.translation.x, soldier_trsf.translation.y);

            // Go to next tile when destination reached
            let rest_x = (soldier_trsf.translation.x.floor() - dest.x).abs();
            let rest_y = (soldier_trsf.translation.y.floor() - dest.y).abs();

            if rest_x < ERROR_MARGIN && rest_y < ERROR_MARGIN {
                soldier.current_tile += 1;
                soldier.dir_set = false;
            }

            // Stop all moves when all tiles covered
            if soldier.current_tile + 1 == soldier.path.len() {
                soldier.move_done = true;
            }
        }
    }
}
