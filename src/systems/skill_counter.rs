use bevy::prelude::*;

use crate::resources::{speed_modifier::SpeedModifier};

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
    pub fn is_ready(&self)->bool{
        self.current >= self.max
    }
}

pub fn skill_tick_sec(
    time: Res<Time>,
    multiplier: Res<SpeedModifier>,
    mut query: Query<&mut SkillCounter>,
) {
    for mut counter in query.iter_mut(){
        if !counter.is_ready(){
            counter.incr(time.delta_seconds_f64() * multiplier.get());
        }
    }
}
