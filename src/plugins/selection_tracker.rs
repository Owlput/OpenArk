use bevy::prelude::{Plugin, App};

use crate::systems::selection_listener::print_events;

pub struct SelectionTrackerPlugin;

impl Plugin for SelectionTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(print_events);
    }
}