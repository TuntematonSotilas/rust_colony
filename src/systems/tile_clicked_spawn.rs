use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::{TilemapGridSize, TilemapType};

use crate::{components::animation_timer::AnimationTimer, resources::cursor_state::CursorState};
use crate::{components::clicked_tile::ClickedTile, utils::position::tile_to_world};

const SPRITE_SIZE: f32 = 20.;
const SPRITE_COL: usize = 4;
const ANIM_DUR: f32 = 0.1;

#[allow(clippy::needless_pass_by_value)]
pub fn tile_clicked_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemap_q: Query<(&Transform, &TilemapGridSize, &TilemapType)>,
    mut cursor_state: ResMut<CursorState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !tilemap_q.is_empty() && !cursor_state.spawn_done {
        if let Some(pos) = cursor_state.click_pos {
            let (map_transform, grid_size, map_type) = tilemap_q.single();

            let world_pos = tile_to_world(pos, *grid_size, *map_type, map_transform);

            let texture_handle = asset_server.load("/public/cursors/clicked.png");
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
                SPRITE_COL,
                1,
                None,
                None,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            commands.spawn((
                ClickedTile,
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
            cursor_state.spawn_done = true;
        }
    }
}
