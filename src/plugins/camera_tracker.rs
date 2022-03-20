use bevy::prelude::*;

use crate::systems::camera_tracker::*;


pub struct CameraTrackerPlugin;

impl Plugin for CameraTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraTracker::new())
        .add_system(track_camera)
        .add_system(sync_entity_with_camera);
    }
}
