use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::ui_menu::MenuState, resources::game_state::GameState};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_menu(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    menu_state: Query<&MenuState>,
    game_state: Res<GameState>,
) -> bool {
    if !game_state.started {
        let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());
        if menu_state.get(state_entity).is_ok() {
            let on_click = OnEvent::new(
                move |In((event_dispatcher_context, _, event, _entity)): In<(
                    EventDispatcherContext,
                    WidgetState,
                    KEvent,
                    Entity,
                )>,
                      mut game_state: ResMut<GameState>| {
                    if let EventType::Click(..) = event.event_type {
                        game_state.started = true;
                    }
                    (event_dispatcher_context, event)
                },
            );

            let parent_id = Some(entity);
            rsx! {
                <ElementBundle>
                    <KButtonBundle
                        button={KButton {
                            text: "Start".into(),
                        }}
                        styles={KStyle {
                            top: Units::Stretch(1.0).into(),
                            bottom: Units::Stretch(1.0).into(),
                            left: Units::Stretch(1.0).into(),
                            right: Units::Stretch(1.0).into(),
                            font_size: StyleProp::Value(20.0),
                            color: StyleProp::Value(Color::hex("#ff0000").unwrap()),
                            cursor: KCursorIcon::default().into(),
                            ..Default::default()
                        }}
                        on_event = {on_click}
                    />
                </ElementBundle>
            };
        }
    }
    true
}
