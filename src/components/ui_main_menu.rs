use bevy::prelude::{Component, Bundle};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct MainMenu {}

impl Widget for MainMenu {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct MainMenuState {}

#[derive(Bundle)]
pub struct MainMenuBundle {
    pub menu: MainMenu,
    pub widget_name: WidgetName,
}

impl Default for MainMenuBundle {
    fn default() -> Self {
        Self {
            menu: MainMenu::default(),
            widget_name: MainMenu::default().get_name(),
        }
    }
}
