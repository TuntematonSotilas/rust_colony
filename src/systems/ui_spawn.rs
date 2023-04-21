
use bevy::{prelude::*, log};
use kayak_ui::prelude::{widgets::*, *};

use crate::resources::game_state::GameState;

#[derive(Default, Clone, PartialEq, Component)]
pub struct MenuButton {
    text: String,
}

#[derive(Bundle)]
pub struct MenuButtonBundle {
    button: MenuButton,
    styles: KStyle,
    on_event: OnEvent,
    widget_name: WidgetName,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            styles: KStyle {
                bottom: Units::Pixels(20.0).into(),
                ..Default::default()
            },
            on_event: OnEvent::default(),
            widget_name: WidgetName("MenuButton".to_string()),
        }
    }
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
        let parent_id = None;
        rsx! {
            <KayakAppBundle>
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
            </KayakAppBundle>
        };

        commands.spawn((widget_context, EventDispatcher::default()));
        
    }

   /* let camera_entity = commands
        .spawn(Camera2dBundle::default())
        .insert(CameraUIKayak)
        .id();*/

    

    /*commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(94.0), Val::Px(30.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("/public/ui/button.png")),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("/public/fonts/the_rift.otf"),
                            font_size: 15.0,
                            color: Color::hex("#ff0000").unwrap(),
                        },
                    ));
                });
        });*/
}
