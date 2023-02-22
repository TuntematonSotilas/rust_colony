use bevy::prelude::*;

use super::tiled_plugin::{TiledMap, TiledMapBundle};

pub struct MapPlugin;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("/public/maps/map.tmx");

     commands.spawn(TiledMapBundle {
         tiled_map: map_handle,
         ..Default::default()
     });

}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}