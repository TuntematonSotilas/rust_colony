use bevy::{prelude::*, log};
use bevy_ecs_tilemap::{tiles::TilePos, prelude::{TilemapGridSize, TilemapType}};

use crate::{resources::{soldiers_state::SoldiersState}, components::soldier::SoldierPos};
use crate::components::{soldier::Soldier};

pub fn soldier_spawn(
	mut commands: Commands, 
	asset_server: Res<AssetServer>,
	tilemap_q: Query<(
		&Transform,
        &TilemapGridSize,
		&TilemapType)>,
	mut soldiers_state: ResMut<SoldiersState>,
) {

	if !tilemap_q.is_empty() && !soldiers_state.spawn_done {

		let (map_transform, grid_size, map_type) = tilemap_q.single();

		let tile_pos = TilePos {
			x: 10,
			y: 10
		};
		
		
		let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
        let transform = *map_transform * Transform::from_translation(tile_center);

		let w_x = transform.translation.x;
		let w_y = transform.translation.y;
		
		log::info!("world_pos: {w_x}/{w_y}");


		commands.spawn((
			Soldier { path: Vec::new(), move_done: true, current_path: 1, current_pos: SoldierPos(10, 10) },
			SpriteBundle {
				texture: asset_server.load("/public/sprites/soldier.png"),
				transform,
				..default()
			}
		));
		soldiers_state.spawn_done = true;
	}
	
}
