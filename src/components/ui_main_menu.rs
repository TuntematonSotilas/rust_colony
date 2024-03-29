use bevy::prelude::{Bundle, Component};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiMainMenu {}

impl Widget for UiMainMenu {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiMainMenuState {}

#[derive(Bundle)]
pub struct UiMainMenuBundle {
    pub ui_main_menu: UiMainMenu,
    pub widget_name: WidgetName,
}

impl Default for UiMainMenuBundle {
    fn default() -> Self {
        Self {
            ui_main_menu: UiMainMenu::default(),
            widget_name: UiMainMenu::default().get_name(),
        }
    }
}
