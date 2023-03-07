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
