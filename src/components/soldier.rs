use bevy::{prelude::*};

#[derive(Component)]
pub struct Soldier 
{
	pub path: Vec<SoldierPos>,
	pub move_done: bool,
	pub current_path: usize,
	pub current_pos: SoldierPos,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SoldierPos(pub u32, pub u32);

impl SoldierPos {
	pub fn successors(&self) -> Vec<SoldierPos> {
	  let &SoldierPos(x, y) = self;
	  vec![SoldierPos(x+1,y)]
	}
  }