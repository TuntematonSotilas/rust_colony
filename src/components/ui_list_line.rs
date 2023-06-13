use bevy::prelude::{Bundle, Component};
use kayak_ui::prelude::{Widget, WidgetName};

use crate::resources::player_state::PlayerMap;

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiListLine {
    pub player_map: PlayerMap,
}

impl Widget for UiListLine {}

#[allow(clippy::module_name_repetitions)]
#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiListLineState {
    pub hovering: bool,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct UiListLineBundle {
    pub ui_list_line: UiListLine,
    pub widget_name: WidgetName,
}

impl Default for UiListLineBundle {
    fn default() -> Self {
        Self {
            ui_list_line: UiListLine::default(),
            widget_name: UiListLine::default().get_name(),
        }
    }
}
