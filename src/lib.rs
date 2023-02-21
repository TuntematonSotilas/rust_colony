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
    
    /*let font = asset_server.load("/public/fonts/arial.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    
    // Demonstrate changing translation
    commands.spawn((Text2dBundle {
        text: Text::from_section("test", text_style).with_alignment(text_alignment),
        ..default()
    },));*/

    commands.spawn(SpriteBundle {
        texture: asset_server.load("/public/maps/tiles.png"),
        ..default()
    });


}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins( DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Tiled Map Editor Example".to_string(),
                    ..Default::default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                asset_folder: "public".to_string(),
                watch_for_changes: true,
                ..default()
            })
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(helpers::tiled::TiledMapPlugin)
        .add_startup_system(setup)
        .run();
}
