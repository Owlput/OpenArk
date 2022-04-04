use bevy::{core::Time, input::Input, math::Vec3, prelude::*};

use crate::systems::selection_tracker::*;
use crate::{general_components::status::Speed, plugins::pickable_movement::ControlEvent};

pub fn pickable_movement_controller(
    mut events: EventWriter<ControlEvent>,
    keyboard: Res<Input<KeyCode>>,
    controller: Query<&PickableMovementController>,
    time: Res<Time>,
    speed_multiplier: Query<&Speed, (With<Selected>, With<Movable>)>,
) {
    if speed_multiplier.is_empty() {
        return;
        //If there's no entity selected that has the Speed component(or entity does not exist),
        //return to prevent errors
        //如果选择的实体没有Speed（或者实体不存在）则直接返回以避免错误
    }
    //the rest of the logic is similar to the free_float_camera
    //剩下的逻辑与free_float_camera类似
    let controller = if let Some(controller) = controller.iter().next() {
        controller
    } else {
        return;
    };
    let PickableMovementController { enabled } = *controller;
    if !enabled {
        return;
    }

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
pub struct PickableMovementController {
    pub enabled: bool,
}
