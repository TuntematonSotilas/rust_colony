use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub started: bool,
    pub map_loaded: bool,
}