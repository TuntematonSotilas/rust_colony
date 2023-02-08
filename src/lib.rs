use bevy::prelude::*;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
