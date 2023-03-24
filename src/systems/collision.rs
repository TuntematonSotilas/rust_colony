use bevy::{prelude::*, log};
use bevy::sprite::collide_aabb::collide;
use bevy_ecs_tilemap::prelude::{TilemapGridSize, TilemapType};
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

use crate::components::{soldier::Soldier, wall::Wall};
use crate::utils::constant::TILE_SIZE;
use crate::utils::position::tile_to_world;

const SOLDIER_SIZE: Vec2 = Vec2::new(20., 20.);
const WALL_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

pub fn collision(
    mut soldier_trsf_q: Query<(&mut Soldier, &Transform)>,
    tiles_q: Query<(&TilePos, &TileTextureIndex)>,
    tilemap_q: Query<(&TilemapGridSize, &TilemapType, &Transform)>,
) {
    if !soldier_trsf_q.is_empty() 
        && !tiles_q.is_empty() 
        && !tilemap_q.is_empty() {

        let (grid_size, map_type, map_transform) = tilemap_q.single();
        let (mut soldier, soldier_trsf) = soldier_trsf_q.single_mut();
        for (tile_pos, texture_index) in &tiles_q {
            if texture_index.0 == 3 {
                let wall_word_pos = tile_to_world(*tile_pos, *grid_size, *map_type, map_transform);
                if collide(soldier_trsf.translation, SOLDIER_SIZE, wall_word_pos, WALL_SIZE).is_some() {
                    log::info!("collide");
                    soldier.move_done = true;
                    soldier.dir_set = false;
                }
            }
            
        }
    }
}