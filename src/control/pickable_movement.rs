use bevy::{prelude::{EventWriter, Res, KeyCode, Query, Component}, input::Input, math::Vec3};

use crate::plugins::pickable_movement::ControlEvent;

pub fn pickable_movement_controller(
    mut events:EventWriter<ControlEvent>,
    keyboard:Res<Input<KeyCode>>,
    controller:Query<&PickableMovementController>
){
    let controller = if let Some(controller) = controller.iter().next() {
        controller
    } else {
        return;
    };
    let PickableMovementController { enabled } = *controller;
    if !enabled{return}

    for (key, dir) in [
        (KeyCode::W, Vec3::Z),
        (KeyCode::A, Vec3::X),
        (KeyCode::S, -Vec3::Z),
        (KeyCode::D, -Vec3::X),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            println!("key pressed");
            events.send(ControlEvent::Translate(dir));
        }
    }
}

#[derive(Component)]
pub struct PickableMovementController{
    pub enabled:bool,
}