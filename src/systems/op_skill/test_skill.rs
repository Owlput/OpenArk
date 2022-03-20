use bevy::prelude::Component;

use crate::{resources::effects::Effect, systems::skill_counter::SkillCounter};

#[derive(Component)]
pub struct TestSkill {
    pub effect: Vec<Effect>,
    pub counter: SkillCounter,
}
