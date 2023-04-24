use bevy::{prelude::*, log};
use kayak_ui::prelude::{widgets::*, *};

use crate::resources::game_state::GameState;

#[derive(Component, Default, Clone, PartialEq)]
pub struct Menu {}

impl Widget for Menu {}

#[derive(Component, Default, Clone, PartialEq)]
pub struct MenuState {}

#[derive(Bundle)]
pub struct MenuBundle {
    pub menu: Menu,
    pub widget_name: WidgetName,
}

impl Default for MenuBundle {
    fn default() -> Self {
        Self {
            menu: Default::default(),
            widget_name: Menu::default().get_name(),
        }
    }
}

fn ui_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    menu_state: Query<&MenuState>,
    game_state: Res<GameState>,
) -> bool {
    
    if !game_state.started {
        let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());
        if menu_state.get(state_entity).is_ok() {

            let on_click = OnEvent::new(
                move |In((event_dispatcher_context, _, event, _entity)): In<(EventDispatcherContext, WidgetState, KEvent, Entity)>,
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
            ui_render);

        let parent_id = None;
        rsx! {
            <KayakAppBundle>
               <MenuBundle />
            </KayakAppBundle>
        };
        
        commands.spawn((widget_context, EventDispatcher::default()));
        
    }
}
