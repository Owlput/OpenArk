use bevy::{
    math::Vec3,
    prelude::{App, EventReader, Plugin, Query, Res, Transform, With},
};

use crate::{control::pickable_movement::*, systems::selection_tracker::*};

#[derive(Default)]
pub struct PickableMovementPlugin;

impl Plugin for PickableMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(control_system)
            .add_event::<ControlEvent>()
            .add_system(pickable_movement_controller);
    }
}

pub enum ControlEvent {
    Translate(Vec3),
}

fn control_system(
    mut events: EventReader<ControlEvent>,
    selected: Res<SelectedMovable>,
    mut query: Query<&mut Transform, With<Selected>>,
) {
    if selected.0 == None {
        return;
    }
    let mut transform = if let Some(transform) = query.iter_mut().next() {
        transform
    } else {
        return;
    };

    for event in events.iter() {
        match event {
            &ControlEvent::Translate(delta) => transform.translation += delta,
        }
    }
}
