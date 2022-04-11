use std::{f32::consts::PI, ops::AddAssign};

use bevy::{
    math::Vec3,
    prelude::{App, EventReader, Plugin, Query, Res, Transform, With},
};
use smooth_bevy_cameras::{LookAngles, LookTransform};

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
#[derive(Clone)]
enum Direction {
    L,
    R,
    F,
    B,
    FL,
    FR,
    BL,
    BR,
    O,
}
impl Direction {
    pub fn to_angle(self) -> f32 {
        match self {
            Direction::L => PI / 2.,
            Direction::R => -PI / 2.,
            Direction::F => 0.,
            Direction::B => PI,
            Direction::FL => PI / 4.,
            Direction::FR => -PI / 4.,
            Direction::BL => 3. * PI / 4.,
            Direction::BR => -3. * PI / 4.,
            Direction::O => 0.,
        }
    }
}
impl AddAssign for Direction {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Direction::O => match rhs {
                Direction::F => *self = Direction::F,
                Direction::B => *self = Direction::B,
                Direction::L => *self = Direction::L,
                Direction::R => *self = Direction::R,
                _ => *self = Direction::O,
            },
            Direction::F => match rhs {
                Direction::B => *self = Direction::O,
                Direction::L => *self = Direction::FL,
                Direction::R => *self = Direction::FR,
                _ => *self = Direction::O,
            },
            Direction::B => match rhs {
                Direction::L => *self = Direction::BL,
                Direction::R => *self = Direction::BR,
                _ => *self = Direction::O,
            },

            Direction::FL => match rhs {
                Direction::R => *self = Direction::F,
                _ => *self = Direction::O,
            },
            Direction::BL => *self = Direction::B,
            Direction::L => *self = Direction::O,
            _ => *self = Direction::O,
        }
        ()
    }
}
pub fn pickable_movement_controller(
    mut events: EventWriter<ControlEvent>,
    camera: Query<&LookTransform, With<CameraToTrack>>,
    target: Query<(&Transform, &Speed), (With<Selected>, With<Movable>)>,
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
    let mut key_pressed_count: f32 = 0.;
    let mut rotation = Direction::O;
    for (key, direction) in [
        (KeyCode::W, Direction::F),
        (KeyCode::S, Direction::B),
        (KeyCode::A, Direction::L),
        (KeyCode::D, Direction::R),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            key_pressed_count += 1.;
            rotation += direction;
        }
    }
    if key_pressed_count <= 0.1 {
        return;
    }
    let wanted = Quat::from_axis_angle(
        Vec3::Y,
        LookAngles::from_vector(camera.get_single().unwrap().look_direction().unwrap()).get_yaw()
            + rotation.to_angle(),
    );
    if let Ok((trans, speed)) = target.get_single() {
        let dir = trans.forward().normalize() * time.delta_seconds() * speed.0;
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

use crate::general_components::status::Speed;

#[derive(Component)]
pub struct PickableMovementController {
    pub enabled: bool,
}
