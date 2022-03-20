use bevy::{
    core::Timer,
    prelude::{App, Plugin},
};

use self::{speed_modifier::SpeedModifier, selection_tracker::{EntitySelected, DisableCameraTranslation}};

pub mod effects;
pub mod selection_tracker;
pub mod speed_modifier;
pub mod ticker;

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.insert_resource(ticker::Tick005(Timer::from_seconds(0.05, true)))
            .insert_resource(SpeedModifier::new(1.0))
            .insert_resource(EntitySelected::default())
            .insert_resource(DisableCameraTranslation::default());
    }
}
