use bevy::prelude::*;

use crate::{
    resources::cursor_state::CursorState,
    systems::{
        mouse_click::mouse_click, tile_clicked_animation::tile_clicked_animation,
        tile_clicked_spawn::tile_clicked_spawn,
    },
};

use super::tiled_plugin::{TiledMap, TiledMapBundle};

pub struct MapPlugin;

#[allow(clippy::needless_pass_by_value)]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("/public/maps/map.tmx");

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..default()
    });
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState {
            spawn_done: false,
            click_pos: None,
        })
        .add_startup_system(setup)
        .add_system(mouse_click)
        .add_system(tile_clicked_spawn)
        .add_system(tile_clicked_animation);
    }
}
