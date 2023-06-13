use bevy::prelude::{Bundle, Component};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiList {}

impl Widget for UiList {}

#[allow(clippy::module_name_repetitions)]
#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiListState {}

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct UiListBundle {
    pub ui_list: UiList,
    pub widget_name: WidgetName,
}

impl Default for UiListBundle {
    fn default() -> Self {
        Self {
            ui_list: UiList::default(),
            widget_name: UiList::default().get_name(),
        }
    }
}
