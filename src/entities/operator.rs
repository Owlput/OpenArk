use bevy::{prelude::Bundle, sprite::SpriteBundle};

use crate::{environment::TransformBundle, general_components::Name};

#[derive(Bundle)]
pub struct Operator{
    name:Name,
    #[bundle]
    transform:TransformBundle,
    #[bundle]
    sprite:SpriteBundle
}