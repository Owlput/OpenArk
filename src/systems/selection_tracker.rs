use bevy::prelude::{info, EventReader, Commands, ResMut};
use bevy_mod_picking::PickingEvent;

use crate::{general_components::Selected, resources::selection_tracker::DisableCameraTranslation};

pub fn selection_tracker(mut commands:Commands,mut events: EventReader<PickingEvent>,mut disable:ResMut<DisableCameraTranslation>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => {
                match e {
                    bevy_mod_picking::SelectionEvent::JustSelected(e) => {
                        disable.0 = true;
                        commands.entity(*e).insert(Selected);
                    },
                    bevy_mod_picking::SelectionEvent::JustDeselected(e) => {
                        disable.0 = false;
                        commands.entity(*e).remove::<Selected>();
                    },
                };
            }
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        }
    }
}