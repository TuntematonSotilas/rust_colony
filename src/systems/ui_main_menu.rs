use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::{ui_main_menu::{MainMenuState}, ui_button::{UiButtonBundle, UiButton}, ui_select::{UiSelectBundle}}, states::game_state::GameState};

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
        
        let state_entity = widget_context.use_state(&mut commands, entity, MainMenuState::default());
        
        if menu_state.get(state_entity).is_ok() {
            
            let image = asset_server.load("/public/ui/menu.png");

            let parent_id = Some(entity);
            rsx! {
                <ElementBundle>
                    {
                        if game_state.0 == GameState::MainMenu {
                            constructor! {
                                <ElementBundle>
                                    <KImageBundle
                                        image={KImage(image)}
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
                                    <UiButtonBundle ui_button={UiButton { text: "SINGLE PLAYER".to_string() }} />
                                </ElementBundle>
                            }
                        } else {
                            constructor! {
                               <ElementBundle
									styles={KStyle{
										layout_type: LayoutType::Grid.into(),
										grid_rows: vec![Units::Pixels(40.), Units::Pixels(40.)].into(),
										grid_cols: vec![Units::Stretch(1.)].into(),
                                        left: Units::Stretch(1.).into(),
                                        right: Units::Stretch(1.).into(),
                                        padding: StyleProp::Value(Edge::new(Units::Pixels(40.),Units::Pixels(0.),Units::Pixels(0.),Units::Pixels(0.))),
										..default()
									}}>
									<ElementBundle
                                        styles={KStyle{
                                            row_index: 0.into(),
                                            col_index: 0.into(),
                                            ..default()
                                        }}>
                                        <UiSelectBundle />
									</ElementBundle>
                                    <ElementBundle
                                        styles={KStyle{
                                            padding: StyleProp::Value(Edge::new(Units::Pixels(40.),Units::Pixels(0.),Units::Pixels(0.),Units::Pixels(0.))),
                                            row_index: 1.into(),
                                            col_index: 0.into(),
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
