use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
