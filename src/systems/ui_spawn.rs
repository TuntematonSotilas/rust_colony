use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    components::{
        ui_button::{UiButton, UiButtonState},
        ui_list::{UiList, UiListState},
        ui_list_line::{UiListLine, UiListLineState},
        ui_main_menu::{UiMainMenu, UiMainMenuBundle, UiMainMenuState},
        ui_select::{UiSelect, UiSelectState}, ui_newgame::{UiNewGame, UiNewGameState},
        ui_hud::{UiHud, UiHudState},
    },
    states::game_state::GameState,
    systems::{
        ui_button::ui_button, ui_list::ui_list, ui_list_line::ui_list_line,
        ui_main_menu::ui_main_menu, ui_select::ui_select, ui_new_game::ui_new_game,
        ui_hud::ui_hud
    },
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

        font_mapping.set_default(asset_server.load("/public/fonts/monorita.kttf"));

        let mut widget_context = KayakRootContext::new(camera_entity);
        widget_context.add_plugin(KayakWidgetsContextPlugin);

        widget_context.add_widget_system(
            UiMainMenu::default().get_name(),
            widget_update::<UiMainMenu, UiMainMenuState>,
            ui_main_menu,
        );

        widget_context.add_widget_system(
            UiNewGame::default().get_name(),
            widget_update::<UiNewGame, UiNewGameState>,
            ui_new_game,
        );

        widget_context.add_widget_system(
            UiHud::default().get_name(),
            widget_update::<UiHud, UiHudState>,
            ui_hud,
        );

        widget_context.add_widget_system(
            UiButton::default().get_name(),
            widget_update::<UiButton, UiButtonState>,
            ui_button,
        );

        widget_context.add_widget_system(
            UiSelect::default().get_name(),
            widget_update::<UiSelect, UiSelectState>,
            ui_select,
        );

        widget_context.add_widget_system(
            UiList::default().get_name(),
            widget_update::<UiList, UiListState>,
            ui_list,
        );

        widget_context.add_widget_system(
            UiListLine::default().get_name(),
            widget_update::<UiListLine, UiListLineState>,
            ui_list_line,
        );

        

        let parent_id = None;
        rsx! {
            <KayakAppBundle>
               <UiMainMenuBundle />
            </KayakAppBundle>
        };

        commands.spawn((widget_context, EventDispatcher::default()));
    }
}
