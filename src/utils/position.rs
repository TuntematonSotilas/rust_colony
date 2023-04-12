use bevy::prelude::{Transform, Vec3};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapType},
    tiles::TilePos,
};

use super::constant::Z_UNIT;

pub fn tile_to_world(
    tile_pos: TilePos,
    grid_size: TilemapGridSize,
    map_type: TilemapType,
    map_transform: &Transform,
) -> Vec3 {
    let tile_center = tile_pos
        .center_in_world(&grid_size, &map_type)
        .extend(Z_UNIT);
    let trsf = *map_transform * Transform::from_translation(tile_center);
    trsf.translation
}
