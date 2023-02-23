use bevy::{prelude::*};

pub fn spawn_soliders(mut commands: Commands, asset_server: Res<AssetServer>) {

	commands.spawn(SpriteBundle {
        texture: asset_server.load("/public/sprites/soldier.png"),
		transform: Transform {
			translation: Vec3::new(0.,0.,10.),
			..default()
		},
        ..default()
    });
}
