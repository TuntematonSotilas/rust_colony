use crate::{
    plugins::tiled_plugin::{TiledMap, TiledMapBundle},
    states::game_state::GameState,
};
use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn map_spawn(
    mut game_state: ResMut<State<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if game_state.0 == GameState::MapLoad {
        game_state.0 = GameState::InGame;
        let map_handle: Handle<TiledMap> = asset_server.load("/public/maps/desert.tmx");
        commands.spawn(TiledMapBundle {
            tiled_map: map_handle,
            ..default()
        });
    }
}
