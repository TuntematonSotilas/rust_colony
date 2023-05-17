use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::{ui_main_menu::{MainMenuState}, ui_button::{UiButtonBundle, UiButton}}, states::game_state::GameState};

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
                                        }} 
                                    />
                                    <UiButtonBundle ui_button={UiButton { text: "SINGLE PLAYER".to_string() }} />
                                </ElementBundle>
                            }
                        } else {
                            //TODO : move this in a new component ui_select
                            constructor! {
                                // <ElementBundle>
                                //     <BackgroundBundle
                                //         styles={KStyle {
                                //             position_type: KPositionType::SelfDirected.into(),
                                //             top: Units::Pixels(0.).into(),
                                //             left: Units::Pixels(0.).into(),
                                //             width: Units::Percentage(100.).into(),
                                //             height: Units::Percentage(100.).into(),
                                //             border: Edge::from((2.,2.,2.,2.)).into(),
                                //             border_color: Color::hex("#414141").unwrap().into(),
                                //             background_color: Color::hex("#000").unwrap().into(),
                                //             ..Default::default()
                                //         }}/>
                                //     <ElementBundle>
                                //         <BackgroundBundle
                                //             styles={KStyle {
                                //                 position_type: KPositionType::ParentDirected.into(),
                                //                 top: Units::Percentage(50.).into(),
                                //                 left: Units::Percentage(40.).into(),
                                //                 width: Units::Percentage(10.).into(),
                                //                 height: Units::Percentage(5.).into(),
                                //                 border: Edge::from((2.,2.,2.,2.)).into(),
                                //                 border_color: Color::hex("#6a6a6a").unwrap().into(),
                                //                 background_color: Color::hex("#000").unwrap().into(),
                                //                 ..Default::default()
                                //             }}>
                                //             <ElementBundle>
                                //                 <TextWidgetBundle
                                //                     styles={KStyle {
                                //                         top: Units::Stretch(1.).into(),
                                //                         left: Units::Stretch(1.).into(),
                                //                         right: Units::Stretch(1.).into(),
                                //                         font_size: (20.).into(),
                                //                         color: Color::hex("#6bf500").unwrap().into(),
                                //                         ..Default::default()
                                //                     }}    
                                //                     text={TextProps {
                                //                         content: "Human".into(),
                                //                         ..Default::default()
                                //                     }}/>
                                //             </ElementBundle>
                                //         </BackgroundBundle>
                                //     </ElementBundle>
                                // </ElementBundle>
								
								<ElementBundle
									styles={KStyle{
										layout_type: LayoutType::Grid.into(),
										grid_rows: vec![Units::Stretch(1.0), Units::Stretch(2.0), Units::Stretch(5.0)].into(),
										grid_cols: vec![Units::Stretch(1.0), Units::Stretch(1.0)].into(),
										..default()
									}}
								>
									<BackgroundBundle
										styles={KStyle{
											background_color: Color::rgb(0.4, 0.9, 0.4).into(),
											color: Color::rgb(0.0, 0.0, 0.0).into(),
											padding: Edge::all(Units::Pixels(5.0)).into(),
											border_radius: Corner::all(10.0).into(),
											row_index: 0.into(),
											col_index: 0.into(),
											col_span: 2.into(),
											layout_type: LayoutType::Row.into(),
											col_between: Units::Pixels(5.0).into(),
											..default()
										}}
									>
										<TextWidgetBundle
											text={TextProps {
												content: "A".into(),
												..default()
											}}
										/>
										<TextWidgetBundle
											text={TextProps {
												content: "B".into(),
												..default()
											}}
										/>
										<TextWidgetBundle
											text={TextProps {
												content: "C".into(),
												..default()
											}}
										/>
										<TextWidgetBundle
											text={TextProps {
												content: "D".into(),
												..default()
											}}
										/>
										<TextWidgetBundle
											text={TextProps {
												content: "E".into(),
												..default()
											}}
										/>

									</BackgroundBundle>
									<TextWidgetBundle
										text={TextProps {
											content: "R1 C0".into(),
											..default()
										}}
										styles={KStyle{
											row_index: 1.into(),
											col_index: 0.into(),
											..default()
										}}
									/>
									<TextWidgetBundle
										text={TextProps {
											content: "R1 C1".into(),
											..default()
										}}
										styles={KStyle{
											row_index: 1.into(),
											col_index: 1.into(),
											..default()
										}}
									/>
									<TextWidgetBundle
										text={TextProps {
											content: "R2 C0".into(),
											..default()
										}}
										styles={KStyle{
											row_index: 2.into(),
											col_index: 0.into(),
											..default()
										}}
									/>
									<TextWidgetBundle
										text={TextProps {
											content: "R2 C1".into(),
											..default()
										}}
										styles={KStyle{
											row_index: 2.into(),
											col_index: 1.into(),
											..default()
										}}
									/>
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
