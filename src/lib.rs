use bevy::prelude::*;
extern crate wasm_bindgen;
use bevy_ecs_tilemap::TilemapPlugin;
use wasm_bindgen::prelude::*;

mod helpers;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn(Camera2dBundle::default());
    
    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("/public/maps/map.tmx");

     commands.spawn(helpers::tiled::TiledMapBundle {
         tiled_map: map_handle,
         ..Default::default()
     });

}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins( DefaultPlugins
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(helpers::tiled::TiledMapPlugin)
        .add_startup_system(setup)
        .run();
}
