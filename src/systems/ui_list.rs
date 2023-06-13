use bevy::{prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{utils::constant::{GREY, BLACK}, components::{ui_list_line::{UiListLineBundle, UiListLine}}, resources::player_state::{PlayerMap}};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_list(
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
                width: Units::Pixels(202.).into(),
                height: Units::Pixels(100.).into(),
                border: Edge::all(1.).into(),
                border_color: Color::hex(GREY).unwrap().into(),
                border_radius: Corner::all(4.).into(),
                background_color: Color::hex(BLACK).unwrap().into(),
                ..Default::default()
            }}> 
            <ElementBundle
                styles={KStyle{
                    layout_type: LayoutType::Grid.into(),
                    grid_rows: vec![Units::Pixels(20.), Units::Pixels(20.)].into(),
                    grid_cols: vec![Units::Stretch(1.)].into(),
                    left: Units::Stretch(1.).into(),
                    right: Units::Stretch(1.).into(),
                    ..default()
                }}>
                <ElementBundle
                    styles={KStyle {
                        row_index: 0.into(),
                        col_index: 0.into(),
                        left: Units::Stretch(1.).into(),
                        right: Units::Stretch(1.).into(),
                        ..default()
                    }}>
                    <UiListLineBundle ui_list_line={UiListLine { player_map: PlayerMap::Desert }} />
                </ElementBundle>
                <ElementBundle
                    styles={KStyle {
                        row_index: 1.into(),
                        col_index: 0.into(),
                        left: Units::Stretch(1.).into(),
                        right: Units::Stretch(1.).into(),
                        ..default()
                    }}>
                    <UiListLineBundle ui_list_line={UiListLine { player_map: PlayerMap::Jungle }} />
                </ElementBundle>
            </ElementBundle>
        </BackgroundBundle>
    };

    true
}
