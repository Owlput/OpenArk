use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct ModelCenter;
#[derive(Component)]
pub struct CenterHandle(pub Entity);
