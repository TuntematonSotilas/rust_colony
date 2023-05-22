use bevy::{prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{states::game_state::GameState, components::ui_select::{UiSelectState, UiSelect}};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_select(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    state_query: Query<&UiSelectState>,
    query: Query<&UiSelect>,
) -> bool {
   
    if let Ok(ui_select) = query.get(entity) {

        let state_entity = widget_context.use_state(&mut commands, entity, UiSelectState::default());

        if let Ok(state) = state_query.get(state_entity) {
        
            let on_event = OnEvent::new(
                move |In(_entity): In<Entity>, 
                    mut event: ResMut<KEvent>,
                    mut game_state: ResMut<State<GameState>>,
                    mut query: Query<&mut UiSelectState>| {
                        if let Ok(mut button) = query.get_mut(state_entity) {
                            match event.event_type {
                                EventType::MouseIn(..) => {
                                    event.stop_propagation();
                                    button.hovering = true;
                                }
                                EventType::MouseOut(..) => {
                                    button.hovering = false;
                                }
                                EventType::Click(..) => {
                                    if game_state.0 == GameState::MainMenu {
                                        game_state.0 = GameState::NewGameMenu;
                                    } else {
                                        game_state.0 = GameState::MapLoad;
                                    }
                                }
                                _ => {} 
                            }
                        }
                    },
            );

            let parent_id = Some(entity);

            let color = match state.hovering {
                true => Color::hex("#ff0000").unwrap(),
                false => Color::hex("#953500").unwrap(),
            };

            rsx! {

                <ElementBundle
                    styles={KStyle{
                        layout_type: LayoutType::Grid.into(),
                        grid_rows: vec![Units::Stretch(1.0)].into(),
                        grid_cols: vec![Units::Stretch(2.0), Units::Stretch(1.0)].into(),
                        ..default()
                    }}>
                    
                    <TextWidgetBundle
                        text={TextProps {
                            content: "Human".into(),
                            ..default()
                        }}
                        styles={KStyle{
                            row_index: 0.into(),
                            col_index: 0.into(),
                            ..default()
                        }}
                    />

                    <TextWidgetBundle
                        text={TextProps {
                            content: "X".into(),
                            ..default()
                        }}
                        styles={KStyle{
                            row_index: 0.into(),
                            col_index: 1.into(),
                            ..default()
                        }}
                    />
                </ElementBundle>
                    
                   /*  <BackgroundBundle
                        styles={KStyle {
                            width: Units::Percentage(100.).into(),
                            height: Units::Percentage(100.).into(),
                            border: Edge::from((2.,2.,2.,2.)).into(),
                            border_color: Color::hex("#414141").unwrap().into(),
                            background_color: Color::hex("#000").unwrap().into(),
                            ..Default::default()
                        }}/> */
                        /* <BackgroundBundle
                            styles={KStyle {
                                top: Units::Percentage(50.).into(),
                                left: Units::Percentage(40.).into(),
                                width: Units::Percentage(10.).into(),
                                height: Units::Percentage(5.).into(),
                                border: Edge::from((2.,2.,2.,2.)).into(),
                                border_color: Color::hex("#6a6a6a").unwrap().into(),
                                background_color: Color::hex("#000").unwrap().into(),
                                ..Default::default()
                            }}> */
                           /*  <ElementBundle>
                                <TextWidgetBundle
                                    styles={KStyle {
                                        top: Units::Stretch(1.).into(),
                                        left: Units::Stretch(1.).into(),
                                        right: Units::Stretch(1.).into(),
                                        font_size: (15.).into(),
                                        color: Color::hex("#6bf500").unwrap().into(),
                                        ..Default::default()
                                    }}    
                                    text={TextProps {
                                        content: "Human".into(),
                                        ..Default::default()
                                    }}/>
                            </ElementBundle> */
            };
        }
    }
    true
}
