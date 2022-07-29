use bevy::{
    gltf::Gltf,
    pbr::{PbrBundle, StandardMaterial},
    prelude::*,
};
use bevy_mod_picking::PickableBundle;
use bevy_rapier3d::prelude::{Collider, ColliderMassProperties, Damping, Friction, Velocity, CoefficientCombineRule};

pub fn setup_environment(
    mut commands: Commands, // Systems have access to commands
    mut meshes: ResMut<Assets<Mesh>>, //We need to "register" those meshes
    mut materials: ResMut<Assets<StandardMaterial>>, // same for materials
    assst_server: Res<AssetServer>,
) {
    let white_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        ..StandardMaterial::default()
    }); // the "add" method returns the handle of the material that was just created.
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        ..Default::default()
    }); // spawn a light in the world in case of darkness
    let center_handle = commands
        .spawn_bundle(TransformBundle::default())
        .insert(ModelCenter)
        .id();
        // We spawn an entity with a "ModelCenter" markup component for queries to filter.
        // markup components are used quite often and come with little performance cost and much convenience, 
        // so feel free to use them to make your life easier.
        // ``.id()`` method returns the entity ID of the entity we just spawned.
        // We will need the ID to attach it to another entity, making it act like the center of its parent.
    commands  // We spawn a cube for testing purposes
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), // A real visible object should have its mesh, which defines its shape, or "skin"
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 5., 0.0), // We need ``Transform`` component to tell the engine where it is.
            ..Default::default() // Just leave everything else to their default value. More modifications will be done later
        })
        .insert_bundle(PickableBundle::default()) // Making it "pickable" provided by ``bevy_mod_picking``
        .insert(PhyMovable) // Making it to move in a more "realistic" way instead of "teleporting".
        .insert(Speed(3.0)) // Define the maximum speed of moving
        .insert(Turning(5.))
        .insert(CenterHandle(center_handle)) // Hold the information that it has a center
        .add_child(center_handle) // Add a child to it using the entity ID of the child we obtained above.
        // ^^^^^^^ define the center for camera to look at
        //确定相机应该看的中心
        .insert(RigidBody::Dynamic) // Making the entity affected by other forces
        .insert(GravityScale(1.0)) // 1x normal gravity
        .insert(Ccd::enabled()) // "Actively" looking for collisions
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z) //Forbid it from spinning on the two axes
        .insert(Velocity {
            linvel: Vec3::new(0., 0., 0.),
            angvel: Vec3::new(0., 0., 0.),
        }) // Insert the ``Velocity`` for the physics engine to work with
        // ^^ rigid body setup ended
        .insert(Collider::cuboid(0.5, 0.5, 0.5)) // Giving it a cuboid collider, or call it hitbox.
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })  // Set the friction coefficient for the phyics engine
        .insert(ColliderMassProperties::Density(1.0)) // Giving it density to calculate the mass
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        // ^^ collider setup ended
        .with_children(|parent| {
            parent.spawn_scene(assst_server.load("test_directioned.gltf#Scene0"));
        });
    commands // set up the plane
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 200f32 })),
            material: white_handle,
            ..PbrBundle::default()
        })
        .insert(RigidBody::Fixed) // Fixed rigid bodies won't be affected by other forces, but it can be teleported
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
