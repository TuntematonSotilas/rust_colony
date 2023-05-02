use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::ui_menu::MenuState, states::game_state::GameState};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_menu(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    menu_state: Query<&MenuState>,
    game_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
) -> bool {
    if game_state.0 == GameState::Menu  {
        
        let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());

        if menu_state.get(state_entity).is_ok() {
        
            let image = asset_server.load("/public/ui/menu.png");

            let on_click = OnEvent::new(
                move |In(_entity): In<Entity>, 
                    event: Res<KEvent>,
                    mut game_state: ResMut<State<GameState>>| {
                        if let EventType::Click(..) = event.event_type {
                            game_state.0 = GameState::MapLoad;
                        }
                    },
            );

            let parent_id = Some(entity);
            rsx! {
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
                </ElementBundle>
            };
        }
    }
    true
}
