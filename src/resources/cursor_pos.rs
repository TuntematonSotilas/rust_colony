use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorPos(pub Vec3);
impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec3::new(-1000.0, -1000.0, 0.0))
    }
}
