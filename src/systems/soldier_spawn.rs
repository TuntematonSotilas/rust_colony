use bevy::{prelude::*, time::Stopwatch};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapType},
    tiles::TilePos,
};

use crate::resources::soldiers_state::SoldiersState;
use crate::{components::soldier::Soldier, utils::pos_util::tile_to_world};

use super::soldier_move::MyTimer;

#[allow(clippy::needless_pass_by_value)]
pub fn soldier_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemap_q: Query<(&Transform, &TilemapGridSize, &TilemapType)>,
    mut soldiers_state: ResMut<SoldiersState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !tilemap_q.is_empty() && !soldiers_state.spawn_done {
        let (map_transform, grid_size, map_type) = tilemap_q.single();

        let tile_pos = TilePos { x: 10, y: 10 };
        let world_pos = tile_to_world(tile_pos, *grid_size, *map_type, map_transform);

        let texture_handle = asset_server.load("/public/sprites/soldier.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(45.0, 45.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn((
            Soldier {
                path: Vec::new(),
                move_done: true,
                current_tile: 0,
                current_pos: Vec2::new(0., 0.),
                init_pos: Some(tile_pos),
            },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: world_pos,
                    ..default()
                },
                ..default()
            },
            MyTimer {
                time: Stopwatch::new(),
            },
        ));
        soldiers_state.spawn_done = true;
    }
}
