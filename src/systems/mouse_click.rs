use bevy::{math::Vec4Swizzles, prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TilePos,
};

use crate::resources::cursor_state::CursorState;

#[allow(clippy::needless_pass_by_value)]
pub fn mouse_click(
    windows_q: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut cursor_state: ResMut<CursorState>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
) {
    if buttons.just_released(MouseButton::Left) && !camera_q.is_empty() && !tilemap_q.is_empty() {
        let window = windows_q.get_single().unwrap();

        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = camera_q.single();

        let (map_size, grid_size, map_type, map_transform) = tilemap_q.single();

        // check if the cursor is inside the window and get its position
        if let Some(screen_position) = window.cursor_position() {
            // check if it is possible to create a ray from screen coordinates to world coordinates
            if let Some(ray) = camera.viewport_to_world(camera_transform, screen_position) {
                // get 2d world mouse coordinates from the ray
                let world_position: Vec2 = ray.origin.truncate();
                let cursor_pos = Vec4::from((world_position, 1.0, 1.0));
                let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                let cursor_in_map_pos_xy = cursor_in_map_pos.xy();

                // Once we have a world position we can transform it into a possible tile position.
                if let Some(tile_pos) =
                    TilePos::from_world_pos(&cursor_in_map_pos_xy, map_size, grid_size, map_type)
                {
                    cursor_state.click_pos = Some(tile_pos);
                    cursor_state.pathfind_done = false;
                    cursor_state.spawn_done = false;
                }
            }
        }
    }
}
