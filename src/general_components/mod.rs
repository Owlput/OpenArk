use bevy::prelude::Component;

pub mod weapon;
pub mod status;
pub mod battle_stats;
pub mod mobility;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub f32,pub f32);//(current,max)