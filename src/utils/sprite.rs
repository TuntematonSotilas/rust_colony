use bevy_ecs_tilemap::tiles::TilePos;

pub fn get_sprite_index(origin: TilePos, dest: TilePos) -> usize {
	if dest.y < origin.y {
		return 0;
	} else if dest.y > origin.y {
		return 1;
	} else if dest.x < origin.x {
		return 2;
	} else if dest.x > origin.x {
		return 3;
	} else {
		return 0;
	}
}