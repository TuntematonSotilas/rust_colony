use bevy::prelude::{Bundle, Component};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiHud {}

impl Widget for UiHud {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiHudState {}

#[derive(Bundle)]
pub struct UiHudBundle {
    pub ui_hud: UiHud,
    pub widget_name: WidgetName,
}

impl Default for UiHudBundle {
    fn default() -> Self {
        Self {
            ui_hud: UiHud::default(),
            widget_name: UiHud::default().get_name(),
        }
    }
}
