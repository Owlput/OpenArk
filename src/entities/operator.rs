use bevy::prelude::{Bundle};

use crate::general_components::*;

#[derive(Bundle)]
pub struct Operator{
    name:Name,
    health:Health,
    weapon:weapon::Weapon,
}

// impl Operator{
//     pub fn new()->Self{
//         Operator
//     }
// }