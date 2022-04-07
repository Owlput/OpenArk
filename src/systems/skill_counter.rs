use bevy::prelude::*;

use crate::resources::{speed_modifier::GameSpeedModifier};

#[derive(Component)]
pub struct HasSkill;
#[derive(Default, Component)]
pub struct SkillCounter {
    pub current: f64,
    pub max: f64,
}
#[allow(dead_code)]
impl SkillCounter {
    pub fn new(inital: f64, max: f64) -> Self {
        SkillCounter {
            current: inital,
            max,
        }
    }
    pub fn incr(&mut self, delta: f64) {
        if self.current + delta <= self.max {
            self.current += delta;
        }
    }
    pub fn is_ready(&self)->bool{
        self.current >= self.max
    }
}

pub fn skill_tick_sec(
    time: Res<Time>,
    multiplier: Res<GameSpeedModifier>,
    mut query: Query<&mut SkillCounter>,
) {
    for mut counter in query.iter_mut(){
        if !counter.is_ready(){
            counter.incr(time.delta_seconds_f64() * multiplier.get());
        }
    }
}
