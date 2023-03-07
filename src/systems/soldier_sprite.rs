use bevy::{log, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::components::soldier::Soldier;
use crate::utils::sprite::get_sprite_index;

#[allow(clippy::needless_pass_by_value)]
pub fn soldier_sprite(
    mut soldier_q: Query<&mut Soldier>,
    mut soldier_sprite_q: Query<&mut TextureAtlasSprite, With<Soldier>>,
) {
    if !soldier_q.is_empty() && !soldier_sprite_q.is_empty() {
        let mut soldier = soldier_q.single_mut();

        if !soldier.move_done && !soldier.dir_set && soldier.path.len() > 1 {
            let mut soldier_sprite = soldier_sprite_q.single_mut();

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

            // Set direction
            soldier_sprite.index = get_sprite_index(origin_tile, dest_tile);

            log::info!("set dir:{}", soldier_sprite.index);

            soldier.dir_set = true;
        }
    }
}
