use bevy::prelude::Resource;

#[derive(Default, Clone, Eq, PartialEq)]
pub enum PlayerRace {
    #[default]
    Human,
    Gray,
}


#[derive(Resource)]
pub struct PlayerState {
    pub player_race: PlayerRace,
}
