use bevy::{prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::ui_select::{UiSelectState}, utils::constant::{DARK_RED, LIGHT_RED, BLACK, GREEN}, resources::player_state::{PlayerRace, PlayerState}};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_select(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    state_query: Query<&UiSelectState>,
    asset_server: Res<AssetServer>,
) -> bool {


    let state_entity = widget_context.use_state(&mut commands, entity, UiSelectState::default());

    if let Ok(state) = state_query.get(state_entity) {
    
        let on_event = OnEvent::new(
            move |In(_entity): In<Entity>, 
                mut event: ResMut<KEvent>,
                mut query: Query<&mut UiSelectState>,
                mut player_state: ResMut<PlayerState>| {
                    if let Ok(mut select) = query.get_mut(state_entity) {
                        match event.event_type {
                            EventType::MouseIn(..) => {
                                event.stop_propagation();
                                select.hovering = true;
                            }
                            EventType::MouseOut(..) => {
                                select.hovering = false;
                            }
                            EventType::Click(..) => {
                                select.player_race = match select.player_race.clone() {
                                    PlayerRace::Human => PlayerRace::Gray,
                                    PlayerRace::Gray => PlayerRace::Human
                               };
                               player_state.player_race = select.player_race.clone();
                            }
                            _ => {} 
                        }
                    }
                },
        );

        let parent_id = Some(entity);

        let color = match state.hovering {
            true => Color::hex(LIGHT_RED).unwrap(),
            false => Color::hex(DARK_RED).unwrap(),
        };

        let pic_name = match state.player_race {
            PlayerRace::Human => "human",
            PlayerRace::Gray => "gray",
        };
        let image: Handle<Image> = asset_server.load(format!("/public/ui/{}.png", pic_name));

        let text = match state.player_race {
            PlayerRace::Human => "Human",
            PlayerRace::Gray => "Gray",
        };

        rsx! {

            <ElementBundle
                styles={KStyle{
                    left: Units::Stretch(1.).into(),
                    right: Units::Stretch(1.).into(),
                    layout_type: LayoutType::Grid.into(),
                    grid_rows: vec![Units::Pixels(25.0)].into(),
                    grid_cols: vec![Units::Pixels(50.0), Units::Pixels(30.0)].into(),
                    ..default()
                }}>
                
                <BackgroundBundle
                    styles={KStyle {
                        row_index: 0.into(),
                        col_index: 0.into(),
                        left: Units::Stretch(1.).into(),
                        right: Units::Stretch(1.).into(),
                        border: Edge::new(1.,0.,1.,1.).into(),
                        border_color: Color::hex(DARK_RED).unwrap().into(),
                        background_color: Color::hex(BLACK).unwrap().into(),
                        ..Default::default()
                    }}> 
                    <TextWidgetBundle
                        styles={KStyle {
                            left: Units::Stretch(1.).into(),
                            right: Units::Stretch(1.).into(),
                            top: Units::Stretch(1.0).into(),
                            bottom: Units::Stretch(1.0).into(),
                            color: Color::hex(GREEN).unwrap().into(),
                            font_size: (12.).into(),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: text.into(),
                            ..default()
                        }} />
                </BackgroundBundle>

                <BackgroundBundle
                    styles={KStyle {
                        row_index: 0.into(),
                        col_index: 1.into(),
                        left: Units::Stretch(1.).into(),
                        right: Units::Stretch(1.).into(),
                        border: Edge::all(1.).into(),
                        border_color: color.into(),
                        border_radius: Corner::new(0.,8.,0.,8.).into(),
                        background_color: Color::hex(BLACK).unwrap().into(),
                        ..Default::default()
                    }}
                    on_event = {on_event}> 
                    
                    <KImageBundle
                        image={KImage(image)}
                        styles={KStyle {
                            position_type: KPositionType::SelfDirected.into(),
                            top: Units::Stretch(1.0).into(),
                            bottom: Units::Stretch(1.0).into(),
                            left: Units::Stretch(1.0).into(),
                            right: Units::Stretch(1.0).into(),
                            width: Units::Pixels(14.).into(),
                            height: Units::Pixels(14.).into(),
                            ..Default::default()
                        }} />

                </BackgroundBundle> 
            </ElementBundle>
                
        };
    }
    
    true
}
