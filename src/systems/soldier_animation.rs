use bevy::prelude::*;

use crate::components::animation_timer::*;

pub fn soldier_animation(
    time: Res<Time>, 
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    if !query.is_empty() {

        let (mut timer, mut sprite) = query.single_mut();

        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = if sprite.index == 3 {
                1
            } else {
                sprite.index + 1
            };
        }
    }
}