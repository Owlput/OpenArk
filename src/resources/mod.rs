use bevy::prelude::{App, Plugin};

use crate::systems::selection_tracker::*;

use self::speed_modifier::GameSpeedModifier;

pub mod effects;
pub mod speed_modifier;
pub mod ticker;

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSpeedModifier::new(1.0))
            .insert_resource(MovableSelectionLock::default())
            .insert_resource(SelectedMovable::default())
            .insert_resource(CameraMode(true));
    }
}

#[derive(Default)]
pub struct CameraMode(pub bool);
