use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{states::game_state::GameState, components::ui_button::UiButtonState};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_button(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    button_state: Query<&UiButtonState>,
) -> bool {
        
    let state_entity = widget_context.use_state(&mut commands, entity, UiButtonState::default());

    if button_state.get(state_entity).is_ok() {
    
        let on_click = OnEvent::new(
            move |In(_entity): In<Entity>, 
                event: Res<KEvent>,
                mut game_state: ResMut<State<GameState>>| {
                    match event.event_type {
                        EventType::Click(..) => {
                            game_state.0 = GameState::MapLoad;
                        }
                        _ => {} 
                    }
                },
        );

        let parent_id = Some(entity);
        rsx! {
            <KButtonBundle
                button={KButton {
                    text: "SINGLE PLAYER".into(),
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
                    color: Color::hex("#ff0000").unwrap().into(),
                    border_color: Color::hex("#ff0000").unwrap().into(),
                    border_radius: Corner::all(0.).into(),
                    cursor: KCursorIcon::default().into(),
                    ..Default::default()
                }}
                on_event = {on_click}
            /> 
        };
    }
    true
}
