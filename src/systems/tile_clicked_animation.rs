use bevy::prelude::*;

use crate::components::{animation_timer::AnimationTimer, clicked_tile::ClickedTile};

#[allow(clippy::needless_pass_by_value)]
pub fn tile_clicked_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut clicked_tile_q: Query<
        (Entity, &mut AnimationTimer, &mut TextureAtlasSprite),
        With<ClickedTile>,
    >,
) {
    if !clicked_tile_q.is_empty() {
        for (entity, mut timer, mut sprite) in &mut clicked_tile_q {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if sprite.index == 3 {
                    commands.entity(entity).despawn();
                } else {
                    sprite.index += 1;
                }
            }
        }
    }
}
