use bevy::prelude::*;

use crate::systems::mouse_click::mouse_click;

use super::tiled_plugin::{TiledMap, TiledMapBundle};

pub struct MapPlugin;

#[allow(clippy::needless_pass_by_value)]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("/public/maps/map.tmx");

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
			.add_system(mouse_click);
    }
}
