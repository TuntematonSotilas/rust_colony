use bevy::prelude::*;

use super::tiled_plugin::{TiledMap, TiledMapBundle};
use crate::{
    resources::cursor_pos::CursorPos,
    systems::{camera_movement::camera_movement, update_cursor_pos::update_cursor_pos},
};

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
        app.init_resource::<CursorPos>()
            .add_startup_system(setup)
            .add_system_to_stage(CoreStage::First, update_cursor_pos.after(camera_movement));
    }
}
