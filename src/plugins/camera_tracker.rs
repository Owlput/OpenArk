use bevy::prelude::*;

use crate::resources::selection_tracker::*;

pub struct CameraTrackerPlugin;

impl Plugin for CameraTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectSelected::default());
    }
}
