use bevy::{prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{utils::constant::{GREY, BLACK}, components::ui_list::UiListState};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_list(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    state_query: Query<&UiListState>,
) -> bool {

    let state_entity = widget_context.use_state(&mut commands, entity, UiListState::default());

    if let Ok(state) = state_query.get(state_entity) {
    
        let parent_id = Some(entity);

        rsx! {
            <BackgroundBundle
                styles={KStyle {
                    left: Units::Stretch(1.).into(),
                    right: Units::Stretch(1.).into(),
                    width: Units::Percentage(100.).into(),
                    height: Units::Pixels(100.).into(),
                    border: Edge::all(1.).into(),
                    border_color: Color::hex(GREY).unwrap().into(),
                    border_radius: Corner::all(4.).into(),
                    background_color: Color::hex(BLACK).unwrap().into(),
                    ..Default::default()
                }}> 
            </BackgroundBundle>
        };
    }
    
    true
}
