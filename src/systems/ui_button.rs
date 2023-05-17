use bevy::{prelude::*, log};
use kayak_ui::prelude::{widgets::*, *};

use crate::{states::game_state::GameState, components::ui_button::{UiButtonState, UiButton}};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_button(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    state_query: Query<&UiButtonState>,
    query: Query<&UiButton>,
) -> bool {
   
    if let Ok(ui_button) = query.get(entity) {

        let state_entity = widget_context.use_state(&mut commands, entity, UiButtonState::default());

        if let Ok(state) = state_query.get(state_entity) {
        
            let on_event = OnEvent::new(
                move |In(_entity): In<Entity>, 
                    mut event: ResMut<KEvent>,
                    mut game_state: ResMut<State<GameState>>,
                    mut query: Query<&mut UiButtonState>| {
                        if let Ok(mut button) = query.get_mut(state_entity) {
                            match event.event_type {
                                EventType::MouseIn(..) => {
                                    event.stop_propagation();
                                    log::info!("mousein");
                                    button.hovering = true;
                                }
                                EventType::MouseOut(..) => {
                                    log::info!("mouseout");
                                    button.hovering = false;
                                }
                                EventType::Click(..) => {
                                    log::info!("clic");
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
                <KButtonBundle
                    button={KButton {
                        text: ui_button.text.clone(),
                    }}
                    styles={KStyle {
                        position_type: KPositionType::SelfDirected.into(),
                        top: Units::Stretch(1.).into(),
                        bottom: Units::Stretch(1.).into(),
                        left: Units::Stretch(1.).into(),
                        right: Units::Stretch(1.).into(),
                        width: Units::Percentage(20.).into(),
                        background_color: Color::hex("#000").unwrap().into(),
                        font_size: (20.).into(),
                        color: color.into(),
                        border_color: color.into(),
                        border_radius: Corner::all(0.).into(),
                        cursor: KCursorIcon::default().into(),
                        ..Default::default()
                    }}
                    on_event = {on_event}
                /> 
            };
        }
    }
    true
}
