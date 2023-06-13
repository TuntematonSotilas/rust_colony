use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    components::{
        ui_button::{UiButton, UiButtonBundle},
        ui_list::UiListBundle,
        ui_main_menu::MainMenuState,
        ui_select::UiSelectBundle,
    },
    states::game_state::GameState,
    utils::constant::SAND,
};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_main_menu(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    menu_state: Query<&MainMenuState>,
    game_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
) -> bool {
    if game_state.0 == GameState::MainMenu || game_state.0 == GameState::NewGameMenu {
        let parent_id = Some(entity);

        let state_entity =
            widget_context.use_state(&mut commands, entity, MainMenuState::default());

        if menu_state.get(state_entity).is_ok() {
            let image: Option<Handle<bevy::prelude::Image>> = if game_state.0 == GameState::MainMenu
            {
                Some(asset_server.load("/public/ui/menu.png"))
            } else {
                None
            };

            rsx! {
                <ElementBundle>
                    {
                        if game_state.0 == GameState::MainMenu && image.is_some() {
                            constructor! {
                                <ElementBundle
                                    styles={KStyle{
                                        left: Units::Stretch(1.).into(),
                                        right: Units::Stretch(1.).into(),
                                        ..default()
                                    }}>
                                    <KImageBundle
                                        image={KImage(image.unwrap())}
                                        styles={KStyle {
                                            position_type: KPositionType::SelfDirected.into(),
                                            top: Units::Stretch(1.0).into(),
                                            bottom: Units::Stretch(1.0).into(),
                                            left: Units::Stretch(1.0).into(),
                                            right: Units::Stretch(1.0).into(),
                                            width: Units::Pixels(640.).into(),
                                            height: Units::Pixels(480.).into(),
                                            ..Default::default()
                                        }} />

                                    <ElementBundle
                                        styles={KStyle{
                                            position_type: KPositionType::SelfDirected.into(),
                                            top: Units::Stretch(1.0).into(),
                                            bottom: Units::Stretch(1.0).into(),
                                            layout_type: LayoutType::Grid.into(),
                                            grid_rows: vec![Units::Pixels(60.), Units::Pixels(60.)].into(),
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
                                                ..Default::default()
                                            }}>
                                            <TextWidgetBundle
                                                styles={KStyle {
                                                    row_index: 0.into(),
                                                    col_index: 0.into(),
                                                    left: Units::Stretch(1.).into(),
                                                    right: Units::Stretch(1.).into(),
                                                    color: Color::hex(SAND).unwrap().into(),
                                                    font_size: (50.).into(),
                                                    ..Default::default()
                                                }}
                                                text={TextProps {
                                                    content: "RUST COLONY".into(),
                                                    ..default()
                                                }} />
                                        </ElementBundle>

                                        <ElementBundle
                                            styles={KStyle{
                                                row_index: 1.into(),
                                                col_index: 0.into(),
                                                left: Units::Stretch(1.).into(),
                                                right: Units::Stretch(1.).into(),
                                                ..default()
                                            }}>
                                            <UiButtonBundle ui_button={UiButton { text: "SINGLE PLAYER".to_string() }} />
                                        </ElementBundle>
                                    </ElementBundle>
                                </ElementBundle>
                            }
                        } else {
                            constructor! {
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
                            }
                        }
                    }
                </ElementBundle>
            };
        }
    }
    true
}
