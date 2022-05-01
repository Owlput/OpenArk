// use std::{f32::consts::PI, ops::AddAssign};

// use bevy::{
//     math::Vec3,
//     prelude::{App, EventReader, Plugin, Query, Res, Transform, With},
// };
// use bevy_rapier3d::{prelude::RigidBodyPositionComponent, na::{ Vector3, UnitQuaternion}};
// use smooth_bevy_cameras::{ LookTransform, LookAngles};

// use crate::{
//     general_components::mobility::Turning,
//     systems::{camera_tracker::CameraToTrack, selection_tracker::*},
// };

// #[derive(Default)]
// pub struct PhyMovementPlugin;

// impl Plugin for PhyMovementPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(control_system)
//             .add_event::<ControlEvent>()
//             .add_system(phy_movement_controller);
//     }
// }

// pub enum ControlEvent {
//     Translate(Vec3, UnitQuaternion<f32>),
// }
// #[derive(Clone, PartialEq)]
// enum Direction {
//     L,
//     R,
//     F,
//     B,
//     FL,
//     FR,
//     BL,
//     BR,
//     O,
// }
// //A pretty dumb way to determine the direction
// //Hoping for a better solution
// //一个判断方向的蠢办法，看看有没有更聪明的
// impl Direction {
//     pub fn to_angle(self) -> f32 {
//         match self {
//             Direction::L => PI / 2.,
//             Direction::R => -PI / 2.,
//             Direction::F => 0.,
//             Direction::B => PI,
//             Direction::FL => PI / 4.,
//             Direction::FR => -PI / 4.,
//             Direction::BL => 3. * PI / 4.,
//             Direction::BR => -3. * PI / 4.,
//             Direction::O => 0.,
//         }
//     }
// }
// impl AddAssign for Direction {
//     //Operator overload
//     //操作符重载
//     //Some cases are not covered here because we simply won't have to deal with them
//     //so we can write less unnecessary code.
//     //没有枚举所有可能，因为有很多情况我们不会遇到，让代码少一点
//     //If it doesn't make sense, then see how we handle keyboard input below
//     //如果还不怎么理解可以看看下面是怎么处理键盘输入的
//     fn add_assign(&mut self, rhs: Self) {
//         match self {
//             Direction::O => match rhs {
//                 Direction::F => *self = Direction::F,
//                 Direction::B => *self = Direction::B,
//                 Direction::L => *self = Direction::L,
//                 Direction::R => *self = Direction::R,
//                 _ => *self = Direction::O,
//             },
//             Direction::F => match rhs {
//                 Direction::B => *self = Direction::O,
//                 Direction::L => *self = Direction::FL,
//                 Direction::R => *self = Direction::FR,
//                 _ => *self = Direction::O,
//             },
//             Direction::B => match rhs {
//                 Direction::L => *self = Direction::BL,
//                 Direction::R => *self = Direction::BR,
//                 _ => *self = Direction::O,
//             },

//             Direction::FL => match rhs {
//                 Direction::R => *self = Direction::F,
//                 _ => *self = Direction::O,
//             },
//             Direction::BL => *self = Direction::B,
//             Direction::L => *self = Direction::O,
//             _ => *self = Direction::O,
//         }
//         ()
//     }
// }
// pub fn phy_movement_controller(
//     mut events: EventWriter<ControlEvent>,
//     camera: Query<&LookTransform, With<CameraToTrack>>,
//     target: Query<(&Transform,&Speed), (With<Selected>, With<PhyMovable>)>,
//     keyboard: Res<Input<KeyCode>>,
//     controller: Query<&PhyMovementController>,
//     time: Res<Time>,
// ) {
//     if target.is_empty() {
//         return;
//         //If there's no entity selected that has the Speed component(or does not exist),
//         //return to prevent errors
//         //如果选择的实体没有Speed（或者实体不存在）则直接返回以避免错误
//     }
//     //the rest of the logic is similar to the camera
//     //剩下的逻辑与相机的实现类似
//     let controller = if let Some(controller) = controller.iter().next() {
//         controller
//     } else {
//         return;
//     };
//     let PhyMovementController { enabled } = *controller;
//     if !enabled {
//         return;
//     }
//     let mut rotation = Direction::O;
//     for (key, direction) in [
//         (KeyCode::W, Direction::F),
//         (KeyCode::S, Direction::B),
//         (KeyCode::A, Direction::L),
//         (KeyCode::D, Direction::R),
//     ]
//     .iter()
//     .cloned()
//     // We first handle back and forth(W/S) and later left and right(A/D)
//     // Take L for example, if we press A then only R will be added to L, no way for F or B
//     // so these cases can be collected with wildcard without causing any issue.
//     //首先处理前后移动再左右移动。
//     //比如我们以向左为例，如果按下了A那L只能再加R,其他的都不可能，所以剩下的就用通配符处理。
//     {
//         if keyboard.pressed(key) {
//             rotation += direction;
//         }
//     }
//     if rotation == Direction::O {
//         return;
//         //If there won't be any movement then just return to avoid the expensive calculation below
//         //如果没有任何移动那就返回以避开下面耗时的计算。
//     }
//     let wanted = UnitQuaternion::from_axis_angle( &Vector3::y_axis(), LookAngles::from_vector(camera.get_single().unwrap().look_direction().unwrap()).get_yaw() + rotation.to_angle());
//     if let Ok((trans,speed)) = target.get_single() {
//         info!("input");
//         events.send(ControlEvent::Translate(-trans.forward().normalize()*speed.0*time.delta_seconds(), wanted));
//     };
// }

// fn control_system(
//     mut events: EventReader<ControlEvent>,
//     selected: Res<SelectedMovable>,
//     mut query_entity: Query<(&mut RigidBodyPositionComponent, &Turning), (With<Selected>, With<Movable>)>,
//     time: Res<Time>,
// ) {
//     if selected.0 == None {
//         return;
//     }
//     if let Some((mut transform, turning_speed)) = query_entity.iter_mut().next() {
//         for event in events.iter() {
//             match event {
//                 &ControlEvent::Translate(trans_step, wanted) => {
//                     let step = turning_speed.0 * time.delta_seconds();
//                     let diff = transform.0.position.rotation.angle_to(&wanted);
//                     let mut stepped = transform.0.position.clone();
//                     stepped.append_rotation_mut(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), step));
//                     if step >= diff {
//                         transform.0.position.rotation = wanted;
//                     } else if wanted.angle_to(&stepped.rotation) < diff {
//                         transform.0.position.rotation = stepped.rotation;
//                     } else {
//                         transform.0.position.append_rotation_mut(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -step))
//                     }
//                     transform.0.position.translation.x += trans_step.x;
//                     transform.0.position.translation.z += trans_step.z;
//                 }
//             }
//         }
//     };
// }

// use bevy::{core::Time, input::Input, prelude::*};

// use crate::general_components::mobility::Speed;

// use super::PhyMovable;

// #[derive(Component)]
// pub struct PhyMovementController {
//     pub enabled: bool,
// }
