use bevy::prelude::{Component, Bundle};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiButton {
    pub is_select: bool,
    pub text: String
}

impl Widget for UiButton {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiButtonState {
    pub hovering: bool,
}

#[derive(Bundle)]
pub struct UiButtonBundle {
    pub ui_button: UiButton,
    pub widget_name: WidgetName,
}

impl Default for UiButtonBundle {
    fn default() -> Self {
        Self {
            ui_button: UiButton::default(),
            widget_name: UiButton::default().get_name(),
        }
    }
}
