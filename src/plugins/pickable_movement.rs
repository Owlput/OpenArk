use bevy::{
    math::Vec3,
    prelude::{App, EventReader, Plugin, Query, Res, Transform, With},
};
use smooth_bevy_cameras::{LookAngles, LookTransform};

use crate::systems::{camera_tracker::CameraToTrack, selection_tracker::*};

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
    mut query_entity: Query<&mut Transform, With<Selected>>,
    query_camera: Query<&LookTransform, With<CameraToTrack>>,
) {
    if selected.0 == None {
        return;
    }
    if let Some(mut transform) = query_entity.iter_mut().next() {
        for event in events.iter() {
            match event {
                &ControlEvent::Translate(delta) => {
                    transform.translation += delta;
                }
            }
        }
        transform.rotation = Quat::from_axis_angle(
            Vec3::Y,
            LookAngles::from_vector(query_camera.get_single().unwrap().look_direction().unwrap())
                .get_yaw(),
        );
    };
}

use bevy::{core::Time, input::Input, prelude::*};

use crate::general_components::status::Speed;

pub fn pickable_movement_controller(
    mut events: EventWriter<ControlEvent>,
    camera: Query<&LookTransform, With<CameraToTrack>>,
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
    let (forward, left) = if let Ok(look_trans) = camera.get_single() {
        let mut ori = look_trans.target - look_trans.eye;
        ori.y = 0.0;
        (
            ori.normalize(),
            Vec3::from_slice(&[ori.z, 0.0, -ori.x]).normalize(),
        )
    } else {
        info!("Failed to get the orientation of the camera");
        return;
    };

    for (key, mut dir) in [
        (KeyCode::W, forward),
        (KeyCode::A, left),
        (KeyCode::S, -forward),
        (KeyCode::D, -left),
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
