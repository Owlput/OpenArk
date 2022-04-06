/*
    The code here is originally from smooth_bevy_camera(MIT)
    We've made some changes to make it suit our need
    代码源自 smooth_bevy_camera
    做了一些调整以达到想要的效果
*/

use crate::{
    resources::CameraMode,
    systems::{camera_tracker::CameraToTrack, selection_tracker::*},
};
use bevy::{math::Vec2, prelude::*};
use smooth_bevy_cameras::*;

#[derive(Default)]
pub struct FreefloatCameraPlugin {
    pub override_input_system: bool,
}

impl FreefloatCameraPlugin {
    pub fn new(override_input_system: bool) -> Self {
        Self {
            override_input_system,
        }
    }
}

impl Plugin for FreefloatCameraPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .add_system(control_system)
            .add_event::<ControlEvent>()
            .add_system(sync_orbit_traget);
        if !self.override_input_system {
            app.add_system(freefloat_cam_controller);
        }
    }
}

#[derive(Bundle)]
pub struct FreefloatCameraBundle {
    controller: FreefloatCameraController,
    #[bundle]
    look_transform: LookTransformBundle,
    #[bundle]
    perspective: PerspectiveCameraBundle,
}
pub enum ControlEvent {
    //Free-float mode only
    //仅自由模式可用
    Rotate(Vec2),
    TranslateEye(Vec3),
    TranslateEyeMouse(f32),
    //Orbit mode only
    //仅轨道相机模式可用
    Zoom(f32),
    ToggleMode(bool),
    Orbit(Vec2),
}
impl FreefloatCameraBundle {
    pub fn new(
        controller: FreefloatCameraController,
        mut perspective: PerspectiveCameraBundle,
        eye: Vec3,
        target: Vec3,
    ) -> Self {
        // Make sure the transform is consistent with the controller to start.
        perspective.transform = Transform::from_translation(eye).looking_at(target, Vec3::Y);

        Self {
            controller,
            look_transform: LookTransformBundle {
                transform: LookTransform { eye, target },
                smoother: Smoother::new(controller.smoothing_weight),
            },
            perspective,
        }
    }
}

pub fn control_system(
    mut cam_ev: EventReader<ControlEvent>,
    mut target_ev: EventReader<crate::plugins::pickable_movement::ControlEvent>,
    mut cameras: Query<(&FreefloatCameraController, &mut LookTransform)>,
    mut cam_mode: ResMut<CameraMode>,
) {
    // Can only control one camera at a time.
    let (controller, mut look_trans) =
        if let Some((controller, look_trans)) = cameras.iter_mut().next() {
            (controller, look_trans) //Looking for the first controller in the query. If there's more than one, it's a bug
                                     //匹配找到的第一个控制器，如果多于一个那就是bug
        } else {
            return; //Controller not found,return
                    //没找到，返回
        };

    if controller.enabled {
        let look_vector = look_trans.look_direction().unwrap();

        if cam_mode.0 {
            let mut look_angles = LookAngles::from_vector(look_vector);
            let yaw_rot = Quat::from_axis_angle(Vec3::Y, look_angles.get_yaw());
            let rot_x = yaw_rot * Vec3::X;
            let rot_y = yaw_rot * Vec3::Y;
            let rot_z = yaw_rot * Vec3::Z;
            for event in cam_ev.iter() {
                match event {
                    //Free-float mode only
                    ControlEvent::Rotate(delta) => {
                        info!("rotate");
                        // Rotates with pitch and yaw.
                        look_angles.add_yaw(-delta.x);
                        look_angles.add_pitch(-delta.y);
                    }
                    ControlEvent::TranslateEye(delta) => {
                        info!("translate eye");
                        // Translates up/down (Y) left/right (X) and forward/back (Z).
                        look_trans.eye += delta.x * rot_x + delta.y * rot_y + delta.z * rot_z;
                    }
                    ControlEvent::TranslateEyeMouse(dis) => {
                        info!("translate eye mouse");
                        let mut ori = look_trans.target - look_trans.eye;
                        ori.y = 0.0;
                        look_trans.eye += ori.normalize() * *dis; //THANK YOU SO MUCH Gibonus#0123@discord for helping out
                    }
                    ControlEvent::ToggleMode(mode) => {
                        info!("change mode to {}", *mode);
                        cam_mode.0 = *mode;
                    }
                    _ => {}
                }
            }
            look_angles.assert_not_looking_up();

            look_trans.target = look_trans.eye + look_trans.radius() * look_angles.unit_vector();
        } else {
            let mut look_angles = LookAngles::from_vector(-look_trans.look_direction().unwrap());
            let mut radius_scalar = 1.0;
            for event in cam_ev.iter() {
                match event {
                    //Orbit mod only
                    ControlEvent::Orbit(delta) => {
                        info!("orbit");
                        look_angles.add_yaw(-delta.x);
                        look_angles.add_pitch(delta.y);
                    }
                    ControlEvent::Zoom(scalar) => {
                        info!("zoom");
                        radius_scalar *= scalar;
                    }
                    ControlEvent::ToggleMode(mode) => {
                        info!("change mode to {}", *mode);
                        cam_mode.0 = *mode;
                    }
                    _ => {}
                };
            }
            info!("lookTrans:{}", look_trans.eye);
            for event in target_ev.iter() {
                match event {
                    super::pickable_movement::ControlEvent::Translate(trans) => {
                        look_trans.eye += *trans
                    }
                }
            }
            look_angles.assert_not_looking_up();

            let new_radius = (radius_scalar * look_trans.radius())
                .min(1000000.0)
                .max(0.001);
            look_trans.eye = look_trans.target + new_radius * look_angles.unit_vector();
        }
    } else {
        drop(cam_ev);
    }
}

use bevy::input::mouse::*;

#[derive(Clone, Component, Copy, Debug)]
pub struct FreefloatCameraController {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub translate_sensitivity: Vec3,
    pub smoothing_weight: f32,
    pub pixels_per_line: f32,
    pub mouse_wheel_zoom_sensitivity: f32,
    pub mode: bool, //true for free_float, false for orbit
}

impl Default for FreefloatCameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(0.002),
            translate_sensitivity: Vec3::splat(0.8),
            smoothing_weight: 0.9,
            pixels_per_line: 53.0,
            mouse_wheel_zoom_sensitivity: 0.5,
            mode: true,
        }
    }
}

pub fn freefloat_cam_controller(
    mut events: EventWriter<ControlEvent>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    controllers: Query<&FreefloatCameraController>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mode: Res<CameraMode>,
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
    if mode.0 {
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
        //check what key has been pressed
        //检查遍历检查按下了什么按键
        {
            if keyboard.pressed(key) {
                events.send(ControlEvent::TranslateEye(translate_sensitivity * dir));
                //If the given key is pressed, send a event to modify the Transform of the camera
                //with the corresponding vector
                //如果有匹配，就发送事件来用对应向量修改相机的Transform
            }
        }

        for event in mouse_wheel_reader.iter() {
            let mut zoom_vec = 5.0;
            let scroll_amount = match event.unit {
                MouseScrollUnit::Line => event.y,
                MouseScrollUnit::Pixel => 0.5,
                //scroll_amount is usually measured with Line, not Pixel.
                //Please report when things go wrong
                //通常scroll_amount都是Line而不是Pixel,如果出现问题请报告
            };
            zoom_vec *= scroll_amount * mouse_wheel_zoom_sensitivity;
            events.send(ControlEvent::TranslateEyeMouse(zoom_vec));
        }
    } else {
        let mut cursor_delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            cursor_delta += event.delta;
        }

        if keyboard.pressed(KeyCode::LControl) {
            info!("orbit");
            events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
        }
        let mut scalar = 1.0;
        for event in mouse_wheel_reader.iter() {
            // scale the event magnitude per pixel or per line
            let scroll_amount = match event.unit {
                MouseScrollUnit::Line => event.y,
                MouseScrollUnit::Pixel => 0.5,
            };
            scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
            events.send(ControlEvent::Zoom(scalar));
        }
    }
}

pub fn sync_orbit_traget(
    selected: Res<SelectedMovable>,
    query_target: Query<&Transform, With<Selected>>,
    mut query_cam: Query<&mut LookTransform, With<CameraToTrack>>,
) {
    if selected.0 == None {
        return;
    }
    match query_target.get_component::<Transform>(selected.0.unwrap()) {
        Ok(target) => query_cam.get_single_mut().unwrap().target = target.translation,
        Err(_) => {
            return;
        }
    };
}
