use bevy::prelude::*;

use crate::{
    components::{animation_timer::AnimationTimer, soldier::Soldier},
    utils::sprite::get_sprite_index_range,
};

#[allow(clippy::needless_pass_by_value)]
pub fn soldier_animation(
    time: Res<Time>,
    soldier_q: Query<&Soldier>,
    mut soldier_anim_q: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    if !soldier_q.is_empty() && !soldier_anim_q.is_empty() {
        let soldier = soldier_q.single();

        if !soldier.move_done && soldier.dir_set {
            let (mut timer, mut sprite) = soldier_anim_q.single_mut();

            let (min, max) = get_sprite_index_range(soldier.direction);

            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                sprite.index = if sprite.index == max {
                    min
                } else {
                    sprite.index + 1
                };
            }
        }
    }
}
