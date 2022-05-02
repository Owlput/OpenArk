use bevy::{prelude::Bundle, sprite::SpriteBundle, transform::TransformBundle};

use crate::general_components::Name;

#[derive(Bundle)]
pub struct Operator {
    name: Name,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    sprite: SpriteBundle,
}
