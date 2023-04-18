use bevy::{prelude::*};

pub fn ui_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
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
        });
}
