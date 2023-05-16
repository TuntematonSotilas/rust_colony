use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{components::{ui_main_menu::{MainMenuState}, ui_button::UiButtonBundle}, states::game_state::GameState};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_main_menu(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    menu_state: Query<&MainMenuState>,
    game_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
) -> bool {
    if game_state.0 == GameState::MainMenu  {
        
        let state_entity = widget_context.use_state(&mut commands, entity, MainMenuState::default());
        
        if menu_state.get(state_entity).is_ok() {
            
            let image = asset_server.load("/public/ui/menu.png");

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
                    <UiButtonBundle />
                </ElementBundle>
            };
        }
    }
    true
}
