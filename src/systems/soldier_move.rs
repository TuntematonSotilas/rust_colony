use bevy::time::Stopwatch;
use bevy::{log, prelude::*};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapType},
    tiles::TilePos,
};

use crate::components::soldier::Soldier;

const SPEED: f32 = 2.;

#[derive(Component)]
pub struct MyTimer {
    pub time: Stopwatch,
}

#[allow(clippy::needless_pass_by_value)]
pub fn soldier_move(
    time: Res<Time>,
    mut soldier_q: Query<&mut Soldier>,
    mut soldier_transform_q: Query<&mut Transform, With<Soldier>>,
    tilemap_q: Query<(&Transform, &TilemapGridSize, &TilemapType), Without<Soldier>>,
) {
    if !soldier_q.is_empty() && !soldier_transform_q.is_empty() && !tilemap_q.is_empty() {
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
                y: soldier.path[soldier.current_tile + 1].1,
            };
            let tile_dest_center = dest_pos.center_in_world(grid_size, map_type).extend(1.0);
            let dest_trsf = *map_transform * Transform::from_translation(tile_dest_center);

            // Get delta from timer
            let delta_x =
                (dest_trsf.translation.x - origin_trsf.translation.x) * time.delta_seconds() * SPEED;
            let delta_y =
                (dest_trsf.translation.y - origin_trsf.translation.y) * time.delta_seconds() * SPEED;

            // Set the position
            let mut soldier_transform = soldier_transform_q.single_mut();
            soldier_transform.translation.x += delta_x;
            soldier_transform.translation.y += delta_y;
			soldier.current_pos = Vec2::new(soldier_transform.translation.x, soldier_transform.translation.y);

            // Go to next tile when destination reached
            let error_margin = 2.;
            if (soldier_transform.translation.x.floor() - dest_trsf.translation.x).abs() < error_margin &&
            (soldier_transform.translation.y.floor() - dest_trsf.translation.y).abs() < error_margin {
                log::info!("next tile");
                soldier.current_tile += 1;
            }

            // Stop all moves when all tiles covered
            if soldier.current_tile + 1 == soldier.path.len() {
                log::info!("move_done");
                soldier.move_done = true;
            }
        }
    }
}
