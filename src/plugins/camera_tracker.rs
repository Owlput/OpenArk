use bevy::prelude::*;

use crate::systems::camera_tracker::*;

/// The plugin for camera tracking setup
pub struct CameraTrackerPlugin;

impl Plugin for CameraTrackerPlugin {
    /// Combine all resources and systems
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraTracker::new())
        .add_system(track_camera)
        .add_system(sync_entity_with_camera);
    }
}