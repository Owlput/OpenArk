use bevy::prelude::{Component, Bundle};

#[derive(Component)]
/// The health points of an entity., namely HP.
pub struct Health(pub f32,pub f32);

#[derive(Component)]
/// The defence points of an entity.  
/// Final loss of HP = Physical attack ponits of the attacker - Def points, a.k.a. substraction.
pub struct Defence(pub f32); 

#[derive(Component)]
/// The resistance percentage points of an entity.  
/// Final loss of HP = Arts attack points of the attacker * (100 - Resistance percentage points), a.k.a. reduce by.  
/// Won't heal when greater than 100.
pub struct Resistance(pub f32); 

#[derive(Bundle)]
pub struct HealthBundle {
    pub hp: Health,
    pub def: Defence,
    pub res: Resistance,
} // Bundle them together for convenience.
  // Works as if we insert them individually.
#[allow(dead_code)]
impl HealthBundle {
    pub fn new(hp: Health, def: Defence, res: Resistance) -> Self {
        Self { hp, def, res }
    }
}
