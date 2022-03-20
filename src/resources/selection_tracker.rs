use bevy::prelude::Entity;
use std::collections::HashSet;
#[derive(Default)]
pub struct EntitySelected(pub HashSet<Entity>);
#[derive(Default)]
pub struct DisableCameraTranslation(pub bool);
impl DisableCameraTranslation{
    pub fn is_disabled(&self)->bool{
        self.0
    }
}
//helper resource
#[derive(Default)]
pub struct InBattle(bool);