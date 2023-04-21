use bevy::{prelude::*, window::WindowResolution};
extern crate wasm_bindgen;
use bevy_ecs_tilemap::TilemapPlugin;
use kayak_ui::{prelude::KayakContextPlugin, widgets::KayakWidgets, CameraUIKayak};
use plugins::{map_plugin::MapPlugin, soldier_plugin::SoldierPlugin, tiled_plugin::TiledMapPlugin, ui_plugin::UiPlugin};
use wasm_bindgen::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;
mod utils;

fn setup(mut commands: Commands) {
    // hide loader
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("loader")
        .unwrap()
        .set_class_name("hide");

    // 2d camera
    commands.spawn(Camera2dBundle::default())
        .insert(CameraUIKayak);

}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    let (w, h) = get_window_size();
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(800., 800.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(SoldierPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(setup)
        .run();
}

#[allow(clippy::cast_possible_truncation)]
fn get_window_size() -> (f32, f32) {
    let w = web_sys::window()
        .unwrap()
        .inner_width()
        .ok()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    let h = web_sys::window()
        .unwrap()
        .inner_height()
        .ok()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    (w, h)
}