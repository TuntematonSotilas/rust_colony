use bevy_ecs_tilemap::tiles::TilePos;

pub const fn get_sprite_index(origin: TilePos, dest: TilePos) -> usize {
    if dest.y > origin.y {
        return 7;
    } else if dest.x < origin.x {
        return 14;
    } else if dest.x > origin.x {
        return 21;
    }
    0
}

pub const fn get_sprite_index_range(direction: usize) -> (usize, usize) {
    if direction == 7 {
        return (8, 13);
    } else if direction == 14 {
        return (15, 20);
    } else if direction == 21 {
        return (22, 27);
    }
    (1, 6)
}
