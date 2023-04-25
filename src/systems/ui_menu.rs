use bevy::{log, prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::menu::MenuState, resources::game_state::GameState};

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
                    match event.event_type {
                        EventType::Click(..) => {
                            log::info!("click");
                            game_state.started = true;
                        }
                        _ => {}
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
                            ..Default::default()
                        }}
                        styles={KStyle {
                            height: Units::Pixels(30.0).into(),
                            left: Units::Stretch(1.0).into(),
                            right: Units::Stretch(1.0).into(),
                            font_size: StyleProp::Value(50.0).into(),
                            color: StyleProp::Value(Color::hex("#ff0000").unwrap()),
                            ..Default::default()
                        }}
                        on_event = {on_click}
                    />
                    <TextWidgetBundle
                        styles={KStyle {
                            color: StyleProp::Value(Color::hex("#ff0000").unwrap()),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Start".into(),
                            size: 20.0,
                            ..Default::default()
                        }}
                    />
                </ElementBundle>
            };
        }
    }
    true
}
