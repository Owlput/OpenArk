use bevy::prelude::{App, Plugin};

use self::speed_modifier::GameSpeedModifier;


pub mod speed_modifier;
pub mod ticker;

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSpeedModifier::new(1.0));
    }
}

#[derive(Default)]
pub struct CameraMode(pub bool);
