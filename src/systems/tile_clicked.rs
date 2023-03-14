use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::{TilemapGridSize, TilemapType};
use crate::{resources::tile_clicked::TileClicked, utils::position::tile_to_world};

#[allow(clippy::needless_pass_by_value)]
pub fn tile_clicked(
	mut commands: Commands,
    mut tile_clicked: Res<TileClicked>,
	asset_server: Res<AssetServer>,
    tilemap_q: Query<(&Transform, &TilemapGridSize, &TilemapType)>,
) {
	if !tilemap_q.is_empty() {
        
		if let Some(pos) = tile_clicked.pos	 {
			let (map_transform, grid_size, map_type) = tilemap_q.single();
			let world_pos = tile_to_world(pos, *grid_size, *map_type, map_transform);
			
			commands.spawn(SpriteBundle {
				texture: asset_server.load("/public/cursors/clicked.png"),
				transform: Transform {
					translation: world_pos,
					..default()
				},
				..default()
			});
		}


	}

	
}