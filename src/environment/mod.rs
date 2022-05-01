use bevy::{
    gltf::Gltf,
    pbr::{PbrBundle, StandardMaterial},
    prelude::*,
};
use bevy_mod_picking::PickableBundle;

#[derive(Bundle, Clone, Copy, Debug, Default)]
pub struct TransformBundle {
    pub local: Transform,
    pub global: GlobalTransform,
}
impl TransformBundle {
    pub fn new(local: Transform) -> Self {
        TransformBundle {
            local,
            global: GlobalTransform::identity(),
        }
    }
}
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
        .spawn_bundle(TransformBundle::new(Transform::from_xyz(0.0, 0.25, 0.0)))
        .insert(ModelCenter)
        .id();
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1., 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Movable)
        .insert(Speed(3.0))
        .insert(Turning(5.))
        .insert(CenterHandle(center_handle))
        .add_child(center_handle)
        .with_children(|parent| {
            parent.spawn_scene(assst_server.load("test_directioned.gltf#Scene0"));
        });
        // .insert_bundle(RigidBodyBundle {
        //     position: Vec3::new(0.0, 2.0, 0.0).into(),
        //     activation: RigidBodyActivation::cannot_sleep().into(),
        //     mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED_X | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z).into(),
        //     ccd: RigidBodyCcd {
        //         ccd_enabled: true,
        //         ..Default::default()
        //     }
        //     .into(),
        //     ..Default::default()
        // })
        // .insert_bundle(ColliderBundle {
        //     shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
        //     collider_type: ColliderType::Solid.into(),
        //     position: (Vec3::new(0.0, 0.0, 0.0), Quat::from_rotation_y(0.0)).into(),
        //     material: ColliderMaterial {
        //         friction: 0.7,
        //         restitution: 0.3,
        //         ..Default::default()
        //     }
        //     .into(),
        //     mass_properties: ColliderMassProps::Density(2.0).into(),
        //     ..Default::default()
        // })
        // .insert(RigidBodyPositionSync::Discrete);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 200f32 })),
            material: white_handle,
            ..PbrBundle::default()
        });
        // .insert_bundle(ColliderBundle {
        //     shape: ColliderShape::cuboid(100.0, 0.05, 100.0).into(),
        //     ..Default::default()
        // });
}

use bevy::gltf::GltfMesh;

use crate::{
    general_components::{
        mobility::Turning,
        model::{CenterHandle, ModelCenter},
        mobility::Speed,
    },rapier_phy::PhyMovable, systems::selection_tracker::Movable,
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
