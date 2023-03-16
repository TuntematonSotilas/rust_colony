use bevy::prelude::Resource;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Resource)]
pub struct CursorState {
    pub spawn_done: bool,
	pub click_pos: Option<TilePos>,
}
