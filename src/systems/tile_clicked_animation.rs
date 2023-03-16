use bevy::prelude::*;

use crate::{
    components::{animation_timer::AnimationTimer, clicked_tile::ClickedTile}
};

#[allow(clippy::needless_pass_by_value)]
pub fn tile_clicked_animation(
    time: Res<Time>,
    mut clicked_tile_q: Query<(&mut ClickedTile, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    if !clicked_tile_q.is_empty() {
        
		for (mut clicked_tile, mut timer, mut sprite) in &mut clicked_tile_q
		{
			if !clicked_tile.anim_done {

				timer.0.tick(time.delta());
				if timer.0.just_finished() {
					if sprite.index == 3 {
						clicked_tile.anim_done = true;
					} else {
						sprite.index += 1;
					}
				}
			}
		}

        
    }
}
