use bevy::prelude::*;

use crate::resources::soldiers_state::SoldiersState;
use crate::systems::soldier_sprite::soldier_sprite;
use crate::systems::{soldier_move::soldier_move, soldier_spawn::soldier_spawn};

pub struct SoldierPlugin;

impl Plugin for SoldierPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoldiersState { spawn_done: false })
            .add_system(soldier_spawn)
            .add_system(soldier_move)
            .add_system(soldier_sprite);
    }
}
