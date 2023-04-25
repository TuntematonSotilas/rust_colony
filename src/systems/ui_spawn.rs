use bevy::{log, prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    components::menu::{Menu, MenuState},
    resources::game_state::GameState,
    systems::ui_menu::ui_menu,
};

#[derive(Bundle)]
pub struct MenuBundle {
    pub menu: Menu,
    pub widget_name: WidgetName,
}

impl Default for MenuBundle {
    fn default() -> Self {
        Self {
            menu: Menu::default(),
            widget_name: Menu::default().get_name(),
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn ui_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
    camera_q: Query<Entity, With<CameraUIKayak>>,
    mut game_state: ResMut<GameState>,
) {
    if !game_state.ui_spawn && !camera_q.is_empty() {
        game_state.ui_spawn = true;

        log::info!("ui_spawn");

        let camera_entity = camera_q.single();

        font_mapping.set_default(asset_server.load("/public/fonts/ace_futurism.kttf"));

        let mut widget_context = KayakRootContext::new(camera_entity);
        widget_context.add_plugin(KayakWidgetsContextPlugin);

        widget_context.add_widget_system(
            Menu::default().get_name(),
            widget_update::<Menu, MenuState>,
            ui_menu,
        );

        let parent_id = None;
        rsx! {
            <KayakAppBundle>
               <MenuBundle />
            </KayakAppBundle>
        };

        commands.spawn((widget_context, EventDispatcher::default()));
    }
}
