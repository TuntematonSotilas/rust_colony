use bevy::prelude::{Component, Bundle};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiButton {}

impl Widget for UiButton {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiButtonState {
    pub hovering: bool,
}

#[derive(Bundle)]
pub struct UiButtonBundle {
    pub menu: UiButton,
    pub widget_name: WidgetName,
}

impl Default for UiButtonBundle {
    fn default() -> Self {
        Self {
            menu: UiButton::default(),
            widget_name: UiButton::default().get_name(),
        }
    }
}
