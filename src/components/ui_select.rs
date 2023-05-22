use bevy::prelude::{Component, Bundle};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiSelect {
    pub text: String
}

impl Widget for UiSelect {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiSelectState {
    pub hovering: bool,
}

#[derive(Bundle)]
pub struct UiSelectBundle {
    pub ui_select: UiSelect,
    pub widget_name: WidgetName,
}

impl Default for UiSelectBundle {
    fn default() -> Self {
        Self {
            ui_select: UiSelect::default(),
            widget_name: UiSelect::default().get_name(),
        }
    }
}
