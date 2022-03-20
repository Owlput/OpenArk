use bevy::{
    pbr::{PbrBundle, StandardMaterial},
    prelude::*, gltf::Gltf,
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
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<bevy::gltf::Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
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
    commands
        .spawn_bundle(TransformBundle::new(Transform::from_xyz(0.0, 0.0, 1.0)))
        .with_children(|parent| {
            // parent.spawn_bundle(PbrBundle{
            //     mesh:asset_server.load("./test_model.gltf#Scene0"),
            //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            //     transform: Transform::from_xyz(5.0, 2.0, 0.0),
            //     ..PbrBundle::default()
            // });
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(crate::general_components::Movable);
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 200f32 })),
                material: white_handle,
                ..PbrBundle::default()
            });
        });
    // commands
    //     .spawn_bundle(TransformBundle::new(Transform::from_xyz(0.0, 0.0, 0.0)))
    //     .with_children(|parent| {
    //         parent.spawn_scene(asset_server.load("./test_model.gltf#Scene0"));
    //     });
    gltf_manual_entity(commands, asset_server.load("./test_model.gltf"), assets_gltf, assets_gltfmesh)
}

use bevy::gltf::GltfMesh;

pub fn gltf_manual_entity(
    mut commands: Commands,
    my: Handle<Gltf>,
    assets_gltf: Res<Assets<bevy::gltf::Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if let Some(gltf) = assets_gltf.get(&my) {
        // Get the GLTF Mesh named "CarWheel"
        // (unwrap safety: we know the GLTF has loaded already)
        let scene = assets_gltfmesh.get(&gltf.named_meshes["Scene0"]).unwrap();

        // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
        commands.spawn_bundle(PbrBundle {
            mesh: scene.primitives[0].mesh.clone(),
            // (unwrap: material is optional, we assume this primitive has one)
            material: scene.primitives[0].material.clone().unwrap(),
            ..Default::default()
        }).insert_bundle(PickableBundle::default());
    }
}
