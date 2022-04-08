mod entities;
mod general_components;
mod systems;
pub mod plugins;
pub mod environment;
pub mod resources;
pub mod event;

use bevy::{prelude::App, DefaultPlugins};
use bevy_obj::ObjPlugin;
use environment::setup_plane;
use plugins::{ tick_skill::TickSkillPlugin, camera::CameraPlugin, selection_tracker::SelectionTrackerPlugin, pickable_movement::PickableMovementPlugin};
use resources::AssetLoader;
use smooth_bevy_cameras::{LookTransformPlugin};
// use crate::plugins::camera::OrbitCameraPlugin;
use systems::{setup_camera};
use bevy_mod_picking::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(LookTransformPlugin)
    .add_plugin(AssetLoader)
    .add_startup_system(setup_camera)
    .add_plugin(CameraPlugin::default())
    .add_startup_system(setup_plane)
    .add_plugins(DefaultPickingPlugins)
    .add_plugin(ObjPlugin)
    .add_plugin(TickSkillPlugin)
    .add_plugin(SelectionTrackerPlugin)
    .add_plugin(PickableMovementPlugin)
    .run();
}