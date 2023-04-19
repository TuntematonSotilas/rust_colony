use bevy::{prelude::*};
use crate::{resources::{game_state::GameState}, plugins::tiled_plugin::{TiledMap, TiledMapBundle}};

#[allow(clippy::needless_pass_by_value)]
pub fn map_spawn(
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    node_q: Query<Entity, &Node>,
) {
    if game_state.started && !game_state.map_loaded {
        game_state.map_loaded = true;
        let map_handle: Handle<TiledMap> = asset_server.load("/public/maps/desert.tmx");
        commands.spawn(TiledMapBundle {
            tiled_map: map_handle,
            ..default()
        });
        for entity in &mut node_q.iter() {
            commands.entity(entity).despawn();
        }
    }
}