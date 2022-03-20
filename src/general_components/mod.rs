use bevy::prelude::Component;

pub mod weapon;
#[derive(Component)]
pub struct Movable;
#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub f32,pub f32);//(current,max)