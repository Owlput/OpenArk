use bevy::prelude::{Plugin, App};

use crate::systems::selection_tracker::*;

pub struct SelectionTrackerPlugin;

impl Plugin for SelectionTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(selection_tracker)
            .add_system(movable_selection_ring_handler)
        ;
    }
}