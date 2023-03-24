use bevy::{prelude::*, log};
use bevy::sprite::collide_aabb::collide;

use crate::components::{soldier::Soldier, wall::Wall};
use crate::utils::constant::{SOLDIER_SPRITE_SIZE, TILE_SIZE};

const SOLDIER_SIZE: Vec2 = Vec2::new(SOLDIER_SPRITE_SIZE, SOLDIER_SPRITE_SIZE);
const WALL_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

pub fn collision(
    soldier_trsf_q: Query<&Transform, With<Soldier>>,
    wall_trsf_q: Query<&Transform, With<Wall>>,
) {
    if !soldier_trsf_q.is_empty() && !wall_trsf_q.is_empty() {
        let soldier_trsf = soldier_trsf_q.single();
        for wall_trsf in &wall_trsf_q {
            if collide(soldier_trsf.translation, SOLDIER_SIZE, wall_trsf.translation, WALL_SIZE).is_some() {
                log::info!("collide");
            }
        }
    }
}