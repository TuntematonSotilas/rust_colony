use bevy_ecs_tilemap::tiles::TilePos;

pub const fn get_sprite_index(origin: TilePos, dest: TilePos) -> usize {
    if dest.y > origin.y {
        return 3;
    } else if dest.x < origin.x {
        return 6;
    } else if dest.x > origin.x {
        return 9;
    }
    0
}

pub const fn get_sprite_index_range(direction: usize) -> (usize, usize) {
    if direction == 3 {
        return (3, 5);
    } else if direction == 6 {
        return (6, 8);
    } else if direction == 9 {
        return (9, 11);
    }
    (0, 2)
}