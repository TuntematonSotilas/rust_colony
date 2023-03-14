use bevy::prelude::Resource;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Resource)]
pub struct TileClicked {
    pub pos: Option<TilePos>,
}
