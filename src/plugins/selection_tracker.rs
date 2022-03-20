use bevy::prelude::{Plugin, App};

use crate::systems::selection_tracker::selection_tracker;

pub struct SelectionTrackerPlugin;

impl Plugin for SelectionTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(selection_tracker);
    }
}