use bevy::prelude::*;

use crate::{
    resources::cursor_state::CursorState,
    systems::{
        map_spawn::map_spawn, mouse_click::mouse_click,
        tile_clicked_animation::tile_clicked_animation, tile_clicked_spawn::tile_clicked_spawn,
    },
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState {
            spawn_done: false,
            pathfind_done: false,
            click_pos: None,
        })
        .add_system(map_spawn)
        .add_system(mouse_click)
        .add_system(tile_clicked_spawn)
        .add_system(tile_clicked_animation);
    }
}
