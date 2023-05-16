use bevy::{prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    components::{ui_main_menu::{MainMenu, MainMenuState, MainMenuBundle}, ui_button::{UiButton, UiButtonState}},
    states::game_state::GameState,
    systems::{ui_main_menu::ui_main_menu, ui_button::ui_button},
};


#[allow(clippy::needless_pass_by_value)]
pub fn ui_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
    camera_q: Query<Entity, With<CameraUIKayak>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if !camera_q.is_empty() && game_state.0 == GameState::MenuLoad {

		game_state.0 = GameState::MainMenu;
		
        let camera_entity = camera_q.single();

        font_mapping.set_default(asset_server.load("/public/fonts/ace_futurism.kttf"));

        let mut widget_context = KayakRootContext::new(camera_entity);
        widget_context.add_plugin(KayakWidgetsContextPlugin);

        widget_context.add_widget_system(
            MainMenu::default().get_name(),
            widget_update::<MainMenu, MainMenuState>,
            ui_main_menu,
        );

        widget_context.add_widget_system(
            UiButton::default().get_name(),
            widget_update::<UiButton, UiButtonState>,
            ui_button,
        );

        let parent_id = None;
        rsx! {
            <KayakAppBundle>
               <MainMenuBundle />
            </KayakAppBundle>
        };

        commands.spawn((widget_context, EventDispatcher::default()));
    }
}
