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

    let w = web_sys::window().unwrap().inner_width().ok().unwrap().as_f64().unwrap() as f32;
    let h = web_sys::window().unwrap().inner_height().ok().unwrap().as_f64().unwrap() as f32;
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins( DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor { 
                    width: w,
                    height: h,
                    ..default()
                },
                ..default()
            })
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(helpers::tiled::TiledMapPlugin)
        .add_startup_system(setup)
        .run();
}
