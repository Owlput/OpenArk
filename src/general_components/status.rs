use bevy::prelude::{Bundle, Component};



#[derive(Component)]
pub struct Health(pub f32,pub f32);//(current,max)
#[derive(Component)]
pub struct Defense(f32);
#[derive(Component)]
pub struct Resistance(f32);


#[derive(Bundle)]
pub struct StatusInBattle{
    health:Health,
    def:Defense,
    res:Resistance
}

#[derive(Component)]
pub struct Speed(pub f32);
#[derive(Component)]
pub struct ShiftResistance(pub f32);

#[derive(Bundle)]
pub struct MobilityStatus{
    speed:Speed,
    shift_res:ShiftResistance
}