use bevy::sprite::collide_aabb::collide;
use bevy::{log, prelude::*};
use bevy_ecs_tilemap::prelude::{TilemapGridSize, TilemapType};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::components::soldier::Soldier;
use crate::utils::constant::{TILE_SIZE, Z_MAP_COLLIDE_LAYER};
use crate::utils::position::tile_to_world;

const SOLDIER_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);
const WALL_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[allow(clippy::needless_pass_by_value)]
pub fn collision(
    mut soldier_trsf_q: Query<(&mut Soldier, &Transform)>,
    tile_q: Query<&TilePos>,
    tilemap_q: Query<(&Transform, &TilemapType, &TilemapGridSize, &TileStorage)>,
) {
    if !soldier_trsf_q.is_empty() && !tilemap_q.is_empty() && !tile_q.is_empty() {
        let (mut soldier, soldier_trsf) = soldier_trsf_q.single_mut();

        for (map_transform, map_type, grid_size, tilemap_storage) in tilemap_q.iter() {
            if map_transform.translation.z == Z_MAP_COLLIDE_LAYER {
                for tile_entity in tilemap_storage.iter().flatten() {
                    if let Ok(tile_pos) = tile_q.get(*tile_entity) {
                        let wall_word_pos =
                            tile_to_world(*tile_pos, *grid_size, *map_type, map_transform);
                        if collide(
                            soldier_trsf.translation,
                            SOLDIER_SIZE,
                            wall_word_pos,
                            WALL_SIZE,
                        )
                        .is_some()
                        {
                            log::info!("collide");
                            soldier.move_done = true;
                            soldier.dir_set = false;
                        }
                    }
                }
            }
        }
    }
}
