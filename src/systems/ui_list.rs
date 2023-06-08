use bevy::{prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{utils::constant::{GREY, BLACK, BLUE}, components::ui_list::UiListState};

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
                    width: Units::Percentage(80.).into(),
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
                    <TextWidgetBundle
                        styles={KStyle {
                            row_index: 0.into(),
                            col_index: 0.into(),
                            left: Units::Stretch(1.).into(),
                            right: Units::Stretch(1.).into(),
                            top: Units::Stretch(1.0).into(),
                            bottom: Units::Stretch(1.0).into(),
                            color: Color::hex(BLUE).unwrap().into(),
                            font_size: (12.).into(),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Jungle (2 Player)".into(),
                            ..default()
                        }} />
                    <TextWidgetBundle
                        styles={KStyle {
                            row_index: 1.into(),
                            col_index: 0.into(),
                            left: Units::Stretch(1.).into(),
                            right: Units::Stretch(1.).into(),
                            top: Units::Stretch(1.0).into(),
                            bottom: Units::Stretch(1.0).into(),
                            color: Color::hex(BLUE).unwrap().into(),
                            font_size: (12.).into(),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Desert (2 Player)".into(),
                            ..default()
                        }} />
                </ElementBundle>
            </BackgroundBundle>
        };
    }
    
    true
}
