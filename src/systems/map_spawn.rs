use crate::{
    plugins::tiled_plugin::{TiledMap, TiledMapBundle},
    states::game_state::GameState, resources::player_state::{PlayerState, PlayerMap},
};
use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn map_spawn(
    mut game_state: ResMut<State<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_state: Res<PlayerState>,
) {
    if game_state.0 == GameState::MapLoad {
        game_state.0 = GameState::InGame;

        let map = if player_state.player_map == PlayerMap::Desert {
            "desert"
        } else {
            "jungle"
        };
        let map_handle: Handle<TiledMap> = asset_server.load(format!("/public/maps/{map}.tmx"));
        
        commands.spawn(TiledMapBundle {
            tiled_map: map_handle,
            ..default()
        });
    }
}
