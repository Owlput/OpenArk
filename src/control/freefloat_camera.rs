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
    //一次仅控制一个摄像机
    let controller = if let Some(controller) = controllers.iter().next() {
        controller
        //Looking for the first controller in the query. If there's more than one, it's a bug
        //匹配找到的第一个控制器，如果多于一个那就是bug
    } else {
        return;
        //Controller not found,return
        //没找到，返回
    };
    let FreefloatCameraController {
        enabled,
        translate_sensitivity,
        mouse_rotate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        ..
    } = *controller;
    //Destructure the controller
    //解构赋值

    if !enabled {
        return;
        //Return if the controller is disabled.
        //如果控制器被禁用了，那就返回
    }

    let mut cursor_delta = Vec2::ZERO; 
    //Initilaize a 2D vector for future use
    //初始化一个二维向量
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
        //Read the event from mouse movement and add the cursor movement 
        //(in 2D since the mouse is moving on your screen, which is a plane) 
        //to the previous 2D vector.
        //读取鼠标移动事件并将鼠标移动向量与先前初始化的向量相加。
    }

    if keyboard.pressed(KeyCode::LControl) {
        //If LControl is pressed, you can look around with your mouse like a FPS-style camera.
        //如果按下了左Ctrl,那你就可以移动鼠标来到处看看。
        events.send(ControlEvent::Rotate(
            mouse_rotate_sensitivity * cursor_delta,
        ));
        //Multiply the movement with the sensitivity in case of moving too fast/slow
        //与敏感度乘积防止移动过快/慢
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
