use bevy::prelude::{Bundle, Component};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiButton {
    pub text: String,
}

impl Widget for UiButton {}

#[allow(clippy::module_name_repetitions)]
#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiButtonState {
    pub hovering: bool,
}

#[allow(clippy::module_name_repetitions)]
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
