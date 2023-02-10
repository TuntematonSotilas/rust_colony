use bevy::prelude::*;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("/public/fonts/arial.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    // 2d camera
    commands.spawn(Camera2dBundle::default());
    // Demonstrate changing translation
    commands.spawn((
        Text2dBundle { 
            text: Text::from_section("test", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
    ));
    
}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
