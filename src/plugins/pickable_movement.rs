use bevy::{
    math::Vec3,
    prelude::{App, EventReader, Plugin, Query, Res, Transform, With},
};
use smooth_bevy_cameras::{LookAngles, LookTransform};

use crate::lib::direction::Direction;
use crate::{
    general_components::mobility::Turning,
    systems::{camera_tracker::CameraToTrack, selection_tracker::*},
};

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
    Translate(Vec3, Quat),
}
pub fn pickable_movement_controller(
    mut events: EventWriter<ControlEvent>,
    camera: Query<&LookTransform, With<CameraToTrack>>,
    target: Query<(&Transform, &Speed), (With<Selected>, With<Movable>)>,
    keyboard: Res<Input<KeyCode>>,
    controller: Query<&PickableMovementController>,
    time: Res<Time>,
) {
    if target.is_empty() {
        return;
        //If there's no entity selected that has the Speed component(or does not exist),
        //return to prevent errors
        //如果选择的实体没有Speed（或者实体不存在）则直接返回以避免错误
    }
    //the rest of the logic is similar to the camera
    //剩下的逻辑与相机的实现类似
    let controller = if let Some(controller) = controller.iter().next() {
        controller
    } else {
        return;
    };
    let PickableMovementController { enabled } = *controller;
    if !enabled {
        return;
    }
    let mut rotation = Direction::O;
    for (key, direction) in [
        (KeyCode::W, Direction::F),
        (KeyCode::S, Direction::B),
        (KeyCode::A, Direction::L),
        (KeyCode::D, Direction::R),
    ]
    .iter()
    .cloned()
    // We first handle back and forth(W/S) and later left and right(A/D)
    // Take L for example, if we press A then only R will be added to L, no way for F or B
    // so these cases can be collected with wildcard without causing any issue.
    //首先处理前后移动再左右移动。
    //比如我们以向左为例，如果按下了A那L只能再加R,其他的都不可能，所以剩下的就用通配符处理。
    {
        if keyboard.pressed(key) {
            rotation += direction;
        }
    }
    if rotation == Direction::O {
        return;
        //If there won't be any movement then just return to avoid the expensive calculation below
        //如果没有任何移动那就返回以避开下面耗时的计算
    }
    let wanted = Quat::from_axis_angle(
        Vec3::Y,
        LookAngles::from_vector(camera.get_single().unwrap().look_direction().unwrap()).get_yaw()
            + rotation.to_angle(),
    );
    if let Ok((trans, speed)) = target.get_single() {
        let dir = trans.forward().normalize() * time.delta_seconds() * speed.0;
        //Calculate the translation here because the event below will also be received
        //by the camera to keep the radius constant
        //We can save a couple of CPU cycle here
        //在这里计算位移是因为同步相机的时候会用到。
        events.send(ControlEvent::Translate(dir, wanted));
    };
}

fn control_system(
    mut events: EventReader<ControlEvent>,
    selected: Res<SelectedMovable>,
    mut query_entity: Query<(&mut Transform, &Turning), (With<Selected>, With<Movable>)>,
    time: Res<Time>,
) {
    if selected.0 == None {
        return;
    }
    if let Some((mut transform, turning_speed)) = query_entity.iter_mut().next() {
        for event in events.iter() {
            match event {
                &ControlEvent::Translate(trans_step, wanted) => {
                    let step = turning_speed.0 * time.delta_seconds();
                    let diff = transform.rotation.angle_between(wanted);
                    let mut stepped = transform.clone();
                    stepped.rotate(Quat::from_axis_angle(Vec3::Y, step));
                    if step >= diff {
                        transform.rotation = wanted;
                    } else if wanted.angle_between(stepped.rotation) < diff {
                        transform.rotation = stepped.rotation;
                    } else {
                        transform.rotate(Quat::from_axis_angle(Vec3::Y, -step))
                    }
                    transform.translation += -trans_step;
                }
            }
        }
    };
}

use bevy::{core::Time, input::Input, prelude::*};

use crate::general_components::mobility::Speed;

#[derive(Component)]
pub struct PickableMovementController {
    pub enabled: bool,
}
