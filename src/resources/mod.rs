use bevy::{
    core::Timer,
    prelude::{App, Plugin},
};

use crate::systems::selection_tracker::*;

use self::speed_modifier::SpeedModifier;

pub mod effects;
pub mod speed_modifier;
pub mod ticker;

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.insert_resource(ticker::Tick005(Timer::from_seconds(0.05, true)))
            .insert_resource(SpeedModifier::new(1.0))
            .insert_resource(MovableSelectionLock::default())
            .insert_resource(SelectedMovable::default())
            .insert_resource(CameraMode(true));
    }
}

#[derive(Default)]
pub struct CameraMode(pub bool);