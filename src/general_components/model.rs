use bevy::prelude::{Component, Entity};

#[derive(Component)]
/// A markup component for marking the entity that represent the center of the model(for camera use)
pub struct ModelCenter;

#[derive(Component)]
/// A componnet that stores the Entity ID of the center of the model.
pub struct CenterHandle(pub Entity);
