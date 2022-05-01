use bevy::prelude::Component;

pub mod weapon;
pub mod status;
pub mod battle_stats;
pub mod mobility;
pub mod model;
pub mod counters;
pub mod effects;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub f32,pub f32);//(current,max)