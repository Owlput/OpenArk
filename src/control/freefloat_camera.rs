use bevy::{input::mouse::*, prelude::*};

use crate::plugins::freefloat_camera::*;

pub fn freefloat_cam_controller(
    mut events: EventWriter<ControlEvent>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    controllers: Query<&FreefloatCameraController>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().next() {
        controller
    } else {
        return;
    };
    let FreefloatCameraController {
        enabled,
        translate_sensitivity,
        mouse_rotate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        ..
    } = *controller;

    if !enabled {
        return;
    }

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    if keyboard.pressed(KeyCode::LControl) {
        events.send(ControlEvent::Rotate(
            mouse_rotate_sensitivity * Vec2::new(cursor_delta.x, cursor_delta.y),
        ));
    }
    for (key, dir) in [
        (KeyCode::W, Vec3::Z),
        (KeyCode::A, Vec3::X),
        (KeyCode::S, -Vec3::Z),
        (KeyCode::D, -Vec3::X),
        (KeyCode::LShift, -Vec3::Y),
        (KeyCode::Space, Vec3::Y),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            events.send(ControlEvent::TranslateEye(translate_sensitivity * dir));
        }
    }

    for event in mouse_wheel_reader.iter() {
        let mut zoom_vec = 5.0;
        // scale the event magnitude per pixel or per line
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => 0.5,
        };
        zoom_vec *= scroll_amount * mouse_wheel_zoom_sensitivity;
        events.send(ControlEvent::TranslateEyeMouse(zoom_vec));
    }
}
