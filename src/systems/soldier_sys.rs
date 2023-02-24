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
	camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {

	if !tilemap_q.is_empty() && !soldiers.set {

		let (grid_size, map_type) = tilemap_q.single();

		let tile_pos = TilePos {
			x: 10,
			y: 10
		};
		
		let world_pos = tile_pos.center_in_world(grid_size, map_type);

		//log::info!("world_pos: {world_pos.x}/{world_pos.y}");

		let world_pos_v3 = Vec3::from((world_pos, 1.0));
		let (camera, camera_transform) = camera_q.single();

		let ndc_pos = camera.world_to_ndc(camera_transform, world_pos_v3).unwrap();

		log::info!(ndc_pos.x);
		log::info!(ndc_pos.y);

		commands.spawn(SpriteBundle {
			texture: asset_server.load("/public/sprites/soldier.png"),
			transform: Transform {
				translation: Vec3::new(ndc_pos.x, ndc_pos.y, 10.),
				..default()
			},
			..default()
		});
		soldiers.set = true;
	}
	
}
