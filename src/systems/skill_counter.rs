use bevy::prelude::*;

use crate::resources::{speed_modifier::SpeedModifier, ticker::Tick005};

#[derive(Component)]
pub struct HasSkill;
#[derive(Default, Component)]
pub struct SkillCounter {
    pub current: f64,
    pub max: f64,
}
impl SkillCounter {
    pub fn new(inital: f64, max: f64) -> Self {
        SkillCounter {
            current: inital,
            max,
        }
    }
    pub fn incr(&mut self, multiplier: f64) {
        let step_in = 0.05 * multiplier;
        if self.current + step_in < self.max {
            self.current += step_in;
        }
    }
}

pub fn skill_tick_sec(
    time: Res<Time>,
    multiplier: Res<SpeedModifier>,
    mut timer: ResMut<Tick005>,
    mut query: Query<&mut SkillCounter>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut counter in query.iter_mut() {
            counter.incr(multiplier.get().into());
        }
    }
}
