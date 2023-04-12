use bevy::{prelude::*};

use crate::systems::ui_spawn::ui_spawn;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_spawn);
    }
}
