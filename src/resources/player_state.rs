use bevy::prelude::Resource;

#[derive(Default, Clone, Eq, PartialEq)]
pub enum PlayerRace {
    #[default]
    Human,
    Gray,
}

#[derive(Default, Clone, Eq, PartialEq)]
pub enum PlayerMap {
    #[default]
    Jungle,
    Desert,
}

#[derive(Resource)]
pub struct PlayerState {
    pub player_race: PlayerRace,
    pub player_map: PlayerMap,
}
