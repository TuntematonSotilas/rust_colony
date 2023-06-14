use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    components::ui_list_line::{UiListLine, UiListLineState},
    resources::player_state::{PlayerMap, PlayerState},
    utils::constant::{BLACK, BLUE, WHITE, DARK_BLUE},
};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_list_line(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    state_query: Query<&UiListLineState>,
    query: Query<&UiListLine>,
    player_state: Res<PlayerState>,
) -> bool {
    if let Ok(ui_list_line) = query.get(entity) {
        let state_entity =
            widget_context.use_state(&mut commands, entity, UiListLineState::default());

        if let Ok(state) = state_query.get(state_entity) {
            let parent_id = Some(entity);

            let map = ui_list_line.player_map.clone();

            let on_event = OnEvent::new(
                move |In(_entity): In<Entity>,
                      mut event: ResMut<KEvent>,
                      mut query: Query<&mut UiListLineState>,
                      mut player_state: ResMut<PlayerState>| {
                    let map = map.clone();

                    if let Ok(mut line) = query.get_mut(state_entity) {
                        match event.event_type {
                            EventType::MouseIn(..) => {
                                event.stop_propagation();
                                line.hovering = true;
                            }
                            EventType::MouseOut(..) => {
                                line.hovering = false;
                            }
                            EventType::Click(..) => {
                                player_state.player_map = map;
                            }
                            _ => {}
                        }
                    }
                },
            );

            let text = match ui_list_line.player_map {
                PlayerMap::Desert => "Desert (2 Player)",
                PlayerMap::Jungle => "Jungle (2 Player)",
            };

            let mut bkg = Color::hex(BLACK).unwrap();
            let mut color = Color::hex(BLUE).unwrap();
            if player_state.player_map == ui_list_line.player_map {
                bkg = Color::hex(DARK_BLUE).unwrap();
                color =  Color::hex(BLACK).unwrap();
            } else if state.hovering {
                color = Color::hex(DARK_BLUE).unwrap()
            };

            rsx! {
                <BackgroundBundle
                    styles={KStyle {
                        left: Units::Stretch(1.).into(),
                        right: Units::Stretch(1.).into(),
                        width: Units::Pixels(200.).into(),
                        background_color: bkg.into(),
                        ..Default::default()
                    }}
                    on_event = {on_event} >
                    <TextWidgetBundle
                        styles={KStyle {
                            top: Units::Stretch(1.).into(),
                            bottom: Units::Stretch(1.).into(),
                            left: Units::Stretch(1.).into(),
                            right: Units::Stretch(1.).into(),
                            width: Units::Pixels(200.).into(),
                            font_size: (12.).into(),
                            color: color.into(),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: text.into(),
                            ..default()
                        }} />
                </BackgroundBundle>
            };
        }
    }

    true
}
