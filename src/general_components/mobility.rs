use bevy::prelude::{Bundle, Component};

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Turning(pub f32, pub f32, pub f32); //(stationary,radius,omega)

#[derive(Component)]
pub struct Force(pub f32);

#[derive(Component)]
pub struct FrictionIndex(pub f32);

#[derive(Component)]
pub struct Weight(pub f32);

#[derive(Component)]
pub struct EscapeIndex(pub f32);

#[derive(Component)]
pub struct RedeployTimer(pub f64, pub f64); //(current,max)

#[derive(Component)]
pub struct ArtsDogeRate(pub f32);

#[derive(Component)]
pub struct PhyDogeRate(pub f32);

#[derive(Component)]
pub struct MaxBlocking(pub u16);

#[derive(Bundle)]
pub struct MobilityBundle {
    speed: Speed,
    turning: Turning,
    force: Force,
    friction_index: FrictionIndex,
    weight: Weight,
    escape_index: EscapeIndex,
    redeploy_timer: RedeployTimer,
    arts_doge_rate: ArtsDogeRate,
    phy_doge_rate: PhyDogeRate,
    max_blocking: MaxBlocking,
}