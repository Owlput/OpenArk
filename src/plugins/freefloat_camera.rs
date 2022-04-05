/*
    The code here is originally from smooth_bevy_camera
*/

use crate::{
    control::freefloat_camera::*,
    resources::CameraMode,
    systems::{
        camera_tracker::CameraToTrack,
        selection_tracker::{Selected, SelectedMovable},
    },
};
use bevy::{math::Vec2, prelude::*};
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

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

#[derive(Component)]
pub struct OrbitTarget;

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
    Rotate(Vec2),
    TranslateEye(Vec3),
    TranslateEyeMouse(f32),
    //Orbit mode only
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
            };
            info!("lookTrans:{}",look_trans.eye);
            for event in target_ev.iter() {
                match event{
                    super::pickable_movement::ControlEvent::Translate(trans) => look_trans.eye += *trans,
                }
            };
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
