use bevy::prelude::{Component, Bundle};

#[derive(Component)]
pub struct Health(pub f32,pub f32);

#[derive(Component)]
pub struct Defence(pub f32);

#[derive(Component)]
pub struct Resistance(pub f32);

#[derive(Bundle)]
pub struct BattleStatsBundle {
    pub hp: Health,
    pub def: Defence,
    pub res: Resistance,
}
#[allow(dead_code)]
impl BattleStatsBundle {
    pub fn new(hp: Health, def: Defence, res: Resistance) -> Self {
        Self { hp, def, res }
    }
}
