use bevy::prelude::Component;
use kayak_ui::prelude::Widget;

#[derive(Component, Default, Clone, PartialEq)]
pub struct Menu {}

impl Widget for Menu {}

#[derive(Component, Default, Clone, PartialEq)]
pub struct MenuState {}
