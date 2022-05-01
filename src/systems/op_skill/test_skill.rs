use bevy::prelude::Component;

use crate::{general_components::{counters::TimeBasedCounter, effects::Effect}};

#[derive(Component)]
pub struct TestSkill {
    pub effect: Vec<Effect>,
    pub counter: TimeBasedCounter,
}
