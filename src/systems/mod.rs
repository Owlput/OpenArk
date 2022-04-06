pub mod camera_tracker;
pub mod op_skill;
pub mod selection_tracker;
pub mod skill_counter;

use bevy::math::Vec3;

use bevy::prelude::{Commands, Component, PerspectiveCameraBundle};

use self::camera_tracker::CameraToTrack;
use crate::plugins::freefloat_camera::*;
use crate::plugins::pickable_movement::PickableMovementController;

#[derive(Component)]
pub struct SyncRotationWithCamera;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(FreefloatCameraBundle::new(
            FreefloatCameraController::default(),
            PerspectiveCameraBundle::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
        ))
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(CameraToTrack);
    commands
        .spawn()
        .insert(PickableMovementController { enabled: true });
}
