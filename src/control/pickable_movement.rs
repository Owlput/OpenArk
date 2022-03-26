use bevy::{prelude::*, input::Input, math::Vec3, core::Time};

use crate::{plugins::pickable_movement::ControlEvent, general_components::status::Speed};
use crate::systems::selection_tracker::*;


pub fn pickable_movement_controller(
    mut events:EventWriter<ControlEvent>,
    keyboard:Res<Input<KeyCode>>,
    controller:Query<&PickableMovementController>,
    time:Res<Time>,
    speed_multiplier:Query<&Speed,(With<Selected>,With<Movable>)>
){
    if speed_multiplier.is_empty(){
        return;
    }
    let controller = if let Some(controller) = controller.iter().next() {
        controller
    } else {
        return;
    };
    let PickableMovementController { enabled } = *controller;
    if !enabled{return}

    for (key, mut dir) in [
        (KeyCode::W, Vec3::Z),
        (KeyCode::A, Vec3::X),
        (KeyCode::S, -Vec3::Z),
        (KeyCode::D, -Vec3::X),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            dir *= speed_multiplier.iter().next().unwrap().0;
            dir *= time.delta().as_secs_f32();
            events.send(ControlEvent::Translate(dir));
        }
    }
}

#[derive(Component)]
pub struct PickableMovementController{
    pub enabled:bool,
}