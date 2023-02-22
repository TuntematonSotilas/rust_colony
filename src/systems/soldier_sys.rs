use bevy::{prelude::*, log};

pub fn spawn_soliders(mut commands: Commands, asset_server: Res<AssetServer>) {

	commands.spawn(SpriteBundle {
        texture: asset_server.load("/public/sprites/soldier.png"),
        ..default()
    });
}
