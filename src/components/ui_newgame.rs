use bevy::prelude::{Bundle, Component};
use kayak_ui::prelude::{Widget, WidgetName};

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiNewGame {}

impl Widget for UiNewGame {}

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct UiNewGameState {}

#[derive(Bundle)]
pub struct UiNewGameBundle {
    pub ui_new_game: UiNewGame,
    pub widget_name: WidgetName,
}

impl Default for UiNewGameBundle {
    fn default() -> Self {
        Self {
            ui_new_game: UiNewGame::default(),
            widget_name: UiNewGame::default().get_name(),
        }
    }
}
