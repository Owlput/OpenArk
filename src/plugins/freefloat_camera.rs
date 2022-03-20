use bevy::{math::Vec2, prelude::*};
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use crate::control::freefloat_camera::freefloat_cam_controller;

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
            .add_event::<ControlEvent>();
        // .add_system(sync_entity_with_camera)

        if !self.override_input_system {
            app.add_system(freefloat_cam_controller);
        }
    }
}

pub enum ControlEvent {
    Rotate(Vec2),
    TranslateEye(Vec3),
    TranslateEyeMouse(f32),
}

#[derive(Bundle)]
pub struct FreefloatCameraBundle {
    controller: FreefloatCameraController,
    #[bundle]
    look_transform: LookTransformBundle,
    #[bundle]
    perspective: PerspectiveCameraBundle,
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

#[derive(Clone, Component, Copy, Debug)]
pub struct FreefloatCameraController {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub translate_sensitivity: f32,
    pub smoothing_weight: f32,
    pub pixels_per_line: f32,
    pub mouse_wheel_zoom_sensitivity: f32,
}

impl Default for FreefloatCameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(0.002),
            translate_sensitivity: 1.0,
            smoothing_weight: 0.9,
            pixels_per_line: 53.0,
            mouse_wheel_zoom_sensitivity: 0.5,
        }
    }
}

pub fn control_system(
    mut events: EventReader<ControlEvent>,
    mut cameras: Query<(&FreefloatCameraController, &mut LookTransform)>,
) {
    // Can only control one camera at a time.
    let (controller, mut look_trans) =
        if let Some((controller, look_trans)) = cameras.iter_mut().next() {
            (controller, look_trans)
        } else {
            return;
        };

    if controller.enabled {
        let look_vector = look_trans.look_direction().unwrap();
        let mut look_angles = LookAngles::from_vector(look_vector);

        let yaw_rot = Quat::from_axis_angle(Vec3::Y, look_angles.get_yaw());
        let rot_x = yaw_rot * Vec3::X;
        let rot_y = yaw_rot * Vec3::Y;
        let rot_z = yaw_rot * Vec3::Z;

        for event in events.iter() {
            match event {
                ControlEvent::Rotate(delta) => {
                    // Rotates with pitch and yaw.
                    look_angles.add_yaw(-delta.x);
                    look_angles.add_pitch(-delta.y);
                }
                ControlEvent::TranslateEye(delta) => {
                    // Translates up/down (Y) left/right (X) and forward/back (Z).
                    look_trans.eye += delta.x * rot_x + delta.y * rot_y + delta.z * rot_z;
                }
                ControlEvent::TranslateEyeMouse(dis) => {
                    let mut ori = look_trans.target - look_trans.eye;
                    ori.y = 0.0;
                    look_trans.eye += ori.normalize() * *dis; //THANK YOU SO MUCH Gibonus#0123@discord for helping out
                }
            }
        }

        look_angles.assert_not_looking_up();

        look_trans.target = look_trans.eye + look_trans.radius() * look_angles.unit_vector();
    } else {
        events.iter(); // Drop the events.
    }
}
