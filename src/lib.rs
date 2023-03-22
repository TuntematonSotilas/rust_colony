use bevy::{prelude::*, window::WindowResolution, log};
extern crate wasm_bindgen;
use bevy_ecs_tilemap::TilemapPlugin;
use plugins::{map_plugin::MapPlugin, soldier_plugin::SoldierPlugin, tiled_plugin::TiledMapPlugin};
use wasm_bindgen::prelude::*;
use bevy_rapier2d::prelude::*;

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
    commands.spawn(Camera2dBundle::default());
}

// ------ ------
//     Start
// ------ ------
/*#[wasm_bindgen(start)]
pub fn start() {
    let (w, h) = get_window_size();
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(w, h),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(SoldierPlugin)
        .add_startup_system(setup)
        .run();
}*/

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
                        resolution: WindowResolution::new(w, h),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(setup_physics)
        .run();

}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}