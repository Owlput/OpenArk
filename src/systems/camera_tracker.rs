
use bevy::prelude::*;

pub fn track_camera(
    mut tracker: ResMut<CameraTracker>,
    query: Query<&Transform, With<CameraToTrack>>,
) {
    if let Ok(transform) = query.get_single() {
        tracker.set_val(transform.translation.clone(), transform.rotation.clone())
    }
}

pub fn sync_entity_with_camera(
    tracker: Res<CameraTracker>,
    mut query: Query<&mut Transform, With<TrackCamera>>,
) {
    for transform in query.iter_mut() {
        transform.looking_at(tracker.get_pos(), Vec3::default());
    }
}

#[derive(Default)]
pub struct CameraTracker {
    pos: Vec3,
    orient: Quat,
}

impl CameraTracker {
    pub fn new() -> Self {
        CameraTracker {
            pos: Vec3::default(),
            orient: Quat::default(),
        }
    }
    pub fn get_pos(&self) -> Vec3 {
        self.pos.clone()
    }
    pub fn set_val(&mut self, pos: Vec3, orient: Quat) {
        self.pos = pos;
        self.orient = orient;
    }
}

#[derive(Component)]
pub struct CameraToTrack;

#[derive(Component)]
pub struct TrackCamera;
