use bevy::prelude::{Bundle, Component};

use super::counters::TimeBasedCounter;

#[derive(Component)]
/// The maximium speed an entity can have, value in meters per second.
pub struct Speed(pub f32);

#[derive(Component)]
/// The maximium angular speed an entity can have, value in rads per second.
pub struct Turning(pub f32); 

#[derive(Component)]
/// The maximium force an entity can apply to other entity, value in newtons.
pub struct Force(pub f32);

#[derive(Component)]
/// The chance of an entity escaping or exiting combat contact, value in percentage.
pub struct EscapeIndex(pub f32); 

#[derive(Component)]
/// The chance of an entity doging a physical attack, value in percentage.
pub struct PhyDogeRate(pub f32);

#[derive(Component)]
/// The chance of an entity doging an arts attack, value in percentage.
pub struct ArtsDogeRate(pub f32); 

#[derive(Component)]
/// The maximium units of entity an entity can hold back.
pub struct MaxBlocking(pub u16);

#[derive(Bundle)]
pub struct MobilityBundle {
    speed: Speed,
    turning: Turning,
    force: Force,
    escape_index: EscapeIndex,
    redeploy_timer: TimeBasedCounter,
    arts_doge_rate: ArtsDogeRate,
    phy_doge_rate: PhyDogeRate,
    max_blocking: MaxBlocking,
}

impl Default for MobilityBundle {
    fn default() -> Self {
        Self {
            speed: Speed(2.0),
            turning: Turning(1.0),
            force: Force(50.0),
            escape_index: EscapeIndex(0.9),
            redeploy_timer: TimeBasedCounter::new(0., 60.),
            arts_doge_rate: ArtsDogeRate(0.0),
            phy_doge_rate: PhyDogeRate(0.0),
            max_blocking: MaxBlocking(2),
        }
    }
}
