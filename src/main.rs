mod entities;
pub mod environment;
pub mod event;
mod general_components;
pub mod plugins;
pub mod rapier_phy;
pub mod resources;
mod systems;
pub mod lib;
pub mod ui;

use bevy::{prelude::App, DefaultPlugins, winit::WinitSettings};
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::{RapierDebugRenderPlugin, DebugRenderStyle, DebugRenderMode};
use environment::setup_plane;
use plugins::*;
use rapier_phy::RapierPhyPlugin;
use resources::AssetLoader;
use smooth_bevy_cameras::LookTransformPlugin;
use systems::setup_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::game())
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
        .add_plugin(RapierDebugRenderPlugin{
           depth_test:false,
           style:DebugRenderStyle::default(),
           mode:DebugRenderMode::all()
        })
        .run();
}
