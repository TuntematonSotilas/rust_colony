use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapType},
    tiles::TilePos,
};

use crate::{components::{animation_timer::AnimationTimer, wall::Wall}, utils::constant::SOLDIER_SPRITE_SIZE};
use crate::resources::soldiers_state::SoldiersState;
use crate::{components::soldier::Soldier, utils::position::tile_to_world};

const SPRITE_COL: usize = 12;
const ANIM_DUR: f32 = 0.1;

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
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(SOLDIER_SPRITE_SIZE, SOLDIER_SPRITE_SIZE),
            SPRITE_COL,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn((
            Soldier {
                path: Vec::new(),
                move_done: true,
                current_tile: 0,
                current_pos: Vec2::new(0., 0.),
                init_pos: Some(tile_pos),
                dir_set: false,
                direction: 0,
            },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: world_pos,
                    ..default()
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(ANIM_DUR, TimerMode::Repeating)),
        ));

        let wall_pos = TilePos { x: 12, y: 12 };
        let wall_world_pos = tile_to_world(wall_pos, *grid_size, *map_type, map_transform);

        commands.spawn((
            Wall,
            SpriteBundle  {
                texture: asset_server.load("/public/sprites/wall.png"),
                transform: Transform {
                    translation: wall_world_pos,
                    ..default()
                },
                ..default()
            },
        ));

        soldiers_state.spawn_done = true;
    }
}
