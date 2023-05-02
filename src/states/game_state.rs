use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
	MenuLoad,
    Menu,
	MapLoad,
    InGame,
}