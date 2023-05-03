use bevy::prelude::{Component, Bundle};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct Menu {}

impl Widget for Menu {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct MenuState {}

#[derive(Bundle)]
pub struct MenuBundle {
    pub menu: Menu,
    pub widget_name: WidgetName,
}

impl Default for MenuBundle {
    fn default() -> Self {
        Self {
            menu: Menu::default(),
            widget_name: Menu::default().get_name(),
        }
    }
}
