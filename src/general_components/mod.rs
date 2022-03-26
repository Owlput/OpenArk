use bevy::prelude::Component;

pub mod weapon;
pub mod status;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub f32,pub f32);//(current,max)