use bevy::{prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::ui_select::{UiSelectState}, utils::constant::{DARK_RED, LIGHT_RED, BLACK, GREEN}};

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

        let pic_human: Handle<Image> = asset_server.load("/public/ui/human.png");


        rsx! {

            <ElementBundle
                styles={KStyle{
                    layout_type: LayoutType::Grid.into(),
                    grid_rows: vec![Units::Stretch(1.0)].into(),
                    grid_cols: vec![Units::Stretch(1.0), Units::Pixels(20.0)].into(),
                    ..default()
                }}>
                
                <BackgroundBundle
                    styles={KStyle {
                        row_index: 0.into(),
                        col_index: 0.into(),
                        border: Edge::from((2.,2.,2.,2.)).into(),
                        border_color: Color::hex(DARK_RED).unwrap().into(),
                        background_color: Color::hex(BLACK).unwrap().into(),
                        ..Default::default()
                    }}> 
                    <TextWidgetBundle
                        styles={KStyle {
                            top: Units::Stretch(1.).into(),
                            bottom: Units::Stretch(1.).into(),
                            left: Units::Stretch(1.).into(),
                            right: Units::Stretch(1.).into(),
                            color: Color::hex(GREEN).unwrap().into(),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Human".into(),
                            ..default()
                        }} />
                </BackgroundBundle>

                <BackgroundBundle
                    styles={KStyle {
                        row_index: 0.into(),
                        col_index: 1.into(),
                        border: Edge::from((2.,2.,2.,2.)).into(),
                        border_color: color.into(),
                        border_radius: Corner::new(0.,10.,0.,10.).into(),
                        background_color: Color::hex(BLACK).unwrap().into(),
                        ..Default::default()
                    }}
                    on_event = {on_event}> 
                    
                    <KImageBundle
                        image={KImage(pic_human)}
                        styles={KStyle {
                            position_type: KPositionType::SelfDirected.into(),
                            top: Units::Stretch(1.0).into(),
                            bottom: Units::Stretch(1.0).into(),
                            left: Units::Stretch(1.0).into(),
                            right: Units::Stretch(1.0).into(),
                            width: Units::Pixels(10.).into(),
                            height: Units::Pixels(10.).into(),
                            ..Default::default()
                        }} />

                </BackgroundBundle> 
            </ElementBundle>
                
        };
    }
    
    true
}
