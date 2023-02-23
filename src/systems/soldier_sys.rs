use bevy::{prelude::*, log};
use bevy_ecs_tilemap::{tiles::{TileStorage, TilePos}, prelude::{TilemapGridSize, TilemapSize, TilemapType}};

use crate::resources::{soldiers::Soldiers};

pub fn spawn_soliders(
	mut commands: Commands, 
	asset_server: Res<AssetServer>,
	tilemap_q: Query<(
        &TilemapGridSize,
		&TilemapType)>,
	mut soldiers: ResMut<Soldiers>,
) {

	if !tilemap_q.is_empty() && !soldiers.set {

		let (grid_size, map_type) = tilemap_q.single();

		let tile_pos = TilePos {
			x: 0,
			y: 0
		};
		
		let pos = tile_pos.center_in_world(grid_size, map_type);

		log::info!(pos.x);

		commands.spawn(SpriteBundle {
			texture: asset_server.load("/public/sprites/soldier.png"),
			transform: Transform {
				translation: Vec3::new(pos.x, pos.y, 10.),
				..default()
			},
			..default()
		});
		soldiers.set = true;
	}
	
}
