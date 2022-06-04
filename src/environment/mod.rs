use bevy::{
    gltf::Gltf,
    pbr::{PbrBundle, StandardMaterial},
    prelude::*,
};
use bevy_mod_picking::PickableBundle;
use bevy_rapier3d::prelude::{Collider, ColliderMassProperties, Damping, Friction, Velocity, CoefficientCombineRule};

pub fn setup_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assst_server: Res<AssetServer>,
) {
    let white_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        ..StandardMaterial::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        ..Default::default()
    });
    let center_handle = commands
        .spawn_bundle(TransformBundle::default())
        .insert(ModelCenter)
        .id();
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 5., 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(PhyMovable)
        .insert(Speed(3.0))
        .insert(Turning(5.))
        .insert(CenterHandle(center_handle))
        .add_child(center_handle)
        // ^^^^^^^ define the center for camera to look at
        //确定相机应该看的中心
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(1.0))
        .insert(Ccd::enabled())
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Velocity {
            linvel: Vec3::new(0., 0., 0.),
            angvel: Vec3::new(0., 0., 0.),
        })
        // ^^ rigid body
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        // ^^ collider
        .with_children(|parent| {
            parent.spawn_scene(assst_server.load("test_directioned.gltf#Scene0"));
        });
    commands // set up the plane
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 200f32 })),
            material: white_handle,
            ..PbrBundle::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100., 0.005, 100.));
}

use bevy::gltf::GltfMesh;
use bevy_rapier3d::prelude::{Ccd, GravityScale, LockedAxes, RigidBody};

use crate::{
    general_components::{
        mobility::Speed,
        mobility::Turning,
        model::{CenterHandle, ModelCenter},
    },
    rapier_phy::PhyMovable,
};

pub fn gltf_manual_entity(
    mut commands: Commands,
    my: Handle<Gltf>,
    assets_gltf: Res<Assets<bevy::gltf::Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if let Some(gltf) = assets_gltf.get(&my) {
        // Get the GLTF Mesh named "CarWheel"
        // (unwrap safety: we know the GLTF has loaded already)
        let scene = assets_gltfmesh.get(&gltf.named_meshes["cube"]).unwrap();

        // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
        commands
            .spawn_bundle(PbrBundle {
                mesh: scene.primitives[0].mesh.clone(),
                // (unwrap: material is optional, we assume this primitive has one)
                material: scene.primitives[0].material.clone().unwrap(),
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default());
    }
}
pub fn gltf_manual_bundle(
    gltf_handle: Handle<Gltf>,
    assets_gltf: &Res<Assets<bevy::gltf::Gltf>>,
    assets_gltfmesh: &Res<Assets<GltfMesh>>,
) -> PbrBundle {
    if let Some(gltf) = assets_gltf.get(gltf_handle) {
        // Get the GLTF Mesh named "CarWheel"
        // (unwrap safety: we know the GLTF has loaded already)
        let scene = assets_gltfmesh.get(&gltf.named_meshes["Scene0"]).unwrap();
        // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
        PbrBundle {
            mesh: scene.primitives[0].mesh.clone(),
            // (unwrap: material is optional, we assume this primitive has one)
            material: scene.primitives[0].material.clone().unwrap(),

            ..Default::default()
        }
    } else {
        PbrBundle::default()
    }
}
