use bevy::prelude::Resource;

#[derive(Resource)]
pub struct SoldiersState {
    pub spawn_done: bool,
}
