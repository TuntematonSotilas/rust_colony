use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub ui_spawn: bool,
    pub started: bool,
    pub map_loaded: bool,
}