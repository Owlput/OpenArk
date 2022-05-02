mod entities;
pub mod environment;
pub mod event;
mod general_components;
pub mod plugins;
pub mod rapier_phy;
pub mod resources;
mod systems;
pub mod lib;

use bevy::{prelude::App, DefaultPlugins};
use bevy_mod_picking::*;
use environment::setup_plane;
use plugins::*;
use rapier_phy::RapierPhyPlugin;
use resources::AssetLoader;
use smooth_bevy_cameras::LookTransformPlugin;
use systems::setup_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(AssetLoader)
        .add_startup_system(setup_camera)
        .add_plugin(CameraPlugin::default())
        .add_startup_system(setup_plane)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(TickSkillPlugin)
        .add_plugin(SelectionTrackerPlugin)
        .add_plugin(PickableMovementPlugin)
        .add_plugin(RapierPhyPlugin)
        .run();
}
