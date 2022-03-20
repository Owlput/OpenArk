use std::collections::VecDeque;

use bevy::{prelude::Entity};

#[derive(Default)]
pub struct ObjectSelected{
    entities:Option<Vec<Entity>>
}
impl ObjectSelected{
    pub fn new()->Self{
        Self{entities:None}
    }
}
#[derive(Default)]
pub struct Operator{
    entities:Option<VecDeque<Entity>>
}

//helper resource
#[derive(Default)]
pub struct InBattle(bool);