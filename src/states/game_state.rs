use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
	MenuLoad,
    MainMenu,
	MapLoad,
    InGame,
}