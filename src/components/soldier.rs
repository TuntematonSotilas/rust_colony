use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
pub struct Soldier {
    pub path: Vec<Pos>,
    pub move_done: bool,
    pub current_tile: usize,
    pub current_pos: Vec2,
    pub init_pos: Option<TilePos>,
    pub dir_set: bool,
    pub direction: usize,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub u32, pub u32);

impl Pos {
    pub fn successors(&self) -> Vec<Self> {
        let &Self(x, y) = self;
        let mut res = vec![Self(x + 1, y), Self(x, y + 1)];
        if x > 0 {
            res.push(Self(x - 1, y));
        }
        if y > 0 {
            res.push(Self(x, y - 1));
        }
        res
    }
}
