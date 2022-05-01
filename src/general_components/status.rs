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
