use bevy::prelude::{Bundle, Component};

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Turning(pub f32); //(omega)

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

impl Default for MobilityBundle{
    fn default()->Self{
        Self{
            speed:Speed(2.0),
            turning:Turning(1.0),
            force:Force(50.0),
            friction_index:FrictionIndex(1.0),
            weight:Weight(60.0),
            escape_index:EscapeIndex(0.9),
            redeploy_timer:RedeployTimer(0.0,50.0),
            arts_doge_rate:ArtsDogeRate(0.0),
            phy_doge_rate:PhyDogeRate(0.0),
            max_blocking:MaxBlocking(2)
        }
    }
}