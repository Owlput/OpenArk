use bevy::prelude::Component;

#[allow(dead_code)]
pub enum DamageType {
    Phys,
    Arts,
    Real,
}
#[allow(dead_code)]
#[derive(Component)]
pub struct Weapon {
    atk: f32,
    damage_type: DamageType,
}
