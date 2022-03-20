pub mod op_skill;
pub mod skill_counter;
pub mod selection_listener;
pub mod camera_tracker;

use bevy::math::{Quat, Vec2, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{
    shape, Assets, BuildChildren, Color, Commands, Component, Mesh, PerspectiveCameraBundle,
    ResMut, Transform,
};

use self::camera_tracker::CameraToTrack;
use self::skill_counter::SkillCounter;
use crate::environment::TransformBundle;
use crate::general_components::Name;
use crate::plugins::freefloat_camera::*;

#[derive(Component)]
pub struct SyncRotationWithCamera;

pub fn add_op(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(TransformBundle::new(
            Transform::from_xyz(2.0, 1.0, -2.0).with_rotation(Quat::default()),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(0.5, 0.5),
                        flip: false,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BLUE,
                        perceptual_roughness: 1.0,
                        ..StandardMaterial::default()
                    }),
                    ..PbrBundle::default()
                })
                .insert(Name("test1".to_string()))
                .insert(SkillCounter::new(10.0, 15.0))
                .insert(SyncRotationWithCamera);
        });
}

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(FreefloatCameraBundle::new(
            FreefloatCameraController::default(),
            PerspectiveCameraBundle::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
        ))
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(CameraToTrack);
}

// pub fn move_selected(pick_state:Res<PickState>,obj_picked:Res<ObjectSelected>,mut entities:Query<&mut Transform,With<PickableBundle>>){
//     for mut transform in entities.iter_mut(){
//         transform.translation = Vec3::new(5.0, -1.0, 3.0);
//     }
// }
