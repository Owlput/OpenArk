use bevy::prelude::Component;

pub mod phy_movement;

use bevy::prelude::Plugin;
use bevy_rapier3d::na::Vector3;
use bevy_rapier3d::plugin::NoUserData;
use bevy_rapier3d::plugin::RapierConfiguration;
use bevy_rapier3d::plugin::RapierPhysicsPlugin;

use self::phy_movement::PhyMovementPlugin;

/// The plugin for enabling Rapier physics engine.
pub struct RapierPhyPlugin;

impl Plugin for RapierPhyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vector3::new(0.0, -9.8, 0.0).into(),
                ..Default::default()
            })
            .add_plugin(PhyMovementPlugin);
    }
}

#[derive(Component)]
/// A markup component used for identifing entities that follow a physical way of moving around.
pub struct PhyMovable;
