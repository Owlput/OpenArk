use bevy::prelude::Component;

#[derive(Component)]
pub struct TimeBasedCounter {
    pub current: f64,
    pub max: f64,
}
#[allow(dead_code)]
impl TimeBasedCounter {
    pub fn new(inital: f64, max: f64) -> Self {
        TimeBasedCounter {
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
