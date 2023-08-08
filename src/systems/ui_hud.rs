use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::utils::constant::{SAND, BLACK};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_hud(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
) -> bool {

    let parent_id = Some(entity);

    rsx! {
    
        <BackgroundBundle
            styles={KStyle {
                left: Units::Stretch(1.).into(),
                right: Units::Stretch(1.).into(),
                width: Units::Stretch(1.).into(),
                height: Units::Stretch(1.).into(),
                border: Edge::all(1.).into(),
                border_color: Color::hex(SAND).unwrap().into(),
                background_color: Color::hex(BLACK).unwrap().into(),
                ..Default::default()
            }}/>
    };
    
    true
}