use bevy::prelude::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

#[derive(Resource)]
pub struct TilesPos {
	pub pos : Vec<Pos>,
	pub set: bool
}