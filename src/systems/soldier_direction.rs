use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::components::soldier::Soldier;
use crate::utils::sprite::get_sprite_index;

#[allow(clippy::needless_pass_by_value)]
pub fn soldier_sprite(mut soldier_sprite_q: Query<(&mut Soldier, &mut TextureAtlasSprite)>) {
    if !soldier_sprite_q.is_empty() {
        let (mut soldier, mut soldier_sprite) = soldier_sprite_q.single_mut();

        if !soldier.dir_set {
            if !soldier.move_done && soldier.path.len() > 1 {
                // Get origin
                let origin_tile = TilePos {
                    x: soldier.path[soldier.current_tile].0,
                    y: soldier.path[soldier.current_tile].1,
                };
                // Get destination
                let dest_tile = TilePos {
                    x: soldier.path[soldier.current_tile + 1].0,
                    y: soldier.path[soldier.current_tile + 1].1,
                };
                // Get direction
                let index = get_sprite_index(origin_tile, dest_tile);
                // Set direction
                soldier_sprite.index = index;
                soldier.direction = index;
            } else {
                // Set direction
                soldier_sprite.index = soldier.direction;
            }
            soldier.dir_set = true;
        }
    }
}
