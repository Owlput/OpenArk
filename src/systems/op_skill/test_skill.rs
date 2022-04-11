use bevy::prelude::Component;

use crate::{resources::effects::Effect, general_components::counters::TimeBasedCounter};

#[derive(Component)]
pub struct TestSkill {
    pub effect: Vec<Effect>,
    pub counter: TimeBasedCounter,
}
