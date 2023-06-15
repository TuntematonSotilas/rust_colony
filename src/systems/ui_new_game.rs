use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::components::{ui_newgame::UiNewGameState, ui_select::UiSelectBundle, ui_list::UiListBundle, ui_button::{UiButtonBundle, UiButton}};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_new_game(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    new_game_state: Query<&UiNewGameState>,
) -> bool {

    let parent_id = Some(entity);

    rsx! {
        <ElementBundle
            styles={KStyle{
                layout_type: LayoutType::Grid.into(),
                grid_rows: vec![Units::Pixels(40.), Units::Pixels(120.), Units::Pixels(40.)].into(),
                grid_cols: vec![Units::Stretch(1.)].into(),
                left: Units::Stretch(1.).into(),
                right: Units::Stretch(1.).into(),
                padding: (Edge::new(Units::Pixels(40.), Units::Pixels(0.), Units::Pixels(0.), Units::Pixels(0.))).into(),
                ..default()
            }}>
            <ElementBundle
                styles={KStyle{
                    row_index: 0.into(),
                    col_index: 0.into(),
                    left: Units::Stretch(1.).into(),
                    right: Units::Stretch(1.).into(),
                    ..default()
                }}>
                <UiSelectBundle />
            </ElementBundle>
            <ElementBundle
                styles={KStyle{
                    row_index: 1.into(),
                    col_index: 0.into(),
                    left: Units::Stretch(1.).into(),
                    right: Units::Stretch(1.).into(),
                    ..default()
                }}>
                <UiListBundle />
            </ElementBundle>
            <ElementBundle
                styles={KStyle{
                    row_index: 2.into(),
                    col_index: 0.into(),
                    left: Units::Stretch(1.).into(),
                    right: Units::Stretch(1.).into(),
                    ..default()
                }}>
                <UiButtonBundle ui_button={UiButton { text: "READY".to_string() }} />
            </ElementBundle>
        </ElementBundle>
    };

    true
}