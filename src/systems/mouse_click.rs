use bevy::{prelude::*, log};

pub fn mouse_click(
	windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
	camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if buttons.just_released(MouseButton::Left) {

		let window = windows.get_primary().unwrap();

    	if let Some(position) = window.cursor_position() {
        	// cursor is inside the window, position given
			log::info!("Left Button was released : {position}");

			// get the camera info and transform
			// assuming there is exactly one main camera entity, so query::single() is OK
			let (camera, camera_transform) = camera_q.single();
			
			// check if the cursor is inside the window and get its position
			if let Some(screen_position) = window.cursor_position() {
				// check if it is possible to create a ray from screen coordinates to world coordinates
				if let Some(ray) = camera.viewport_to_world(camera_transform, screen_position) {
					// get 2d world mouse coordinates from the ray
					let world_position: Vec2 = ray.origin.truncate();
		
					log::info!("World coords: {world_position}");
				}
			}
			
    	} 

    }
}
