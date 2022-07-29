use bevy::prelude::Component;

#[derive(Component)]
/// A counter that increases with time, controlled by a system.
/// Time elapsed is provided by bevy engine.
pub struct TimeBasedCounter {
    current: f64,
    max: f64,
    // Not public field in case of accidental modification and break some stuff.
}
pub enum CounterStat {
    IncrSuccess,
    MaxReached,
}
#[allow(dead_code)]
impl TimeBasedCounter {
    pub fn new(inital: f64, max: f64) -> Self {
        TimeBasedCounter {
            current: inital,
            max,
        }
    }
    pub fn incr(&mut self, delta: f64) -> CounterStat {
        if self.current >= self.max {
            return CounterStat::MaxReached;
        }
        self.current += delta;
        CounterStat::IncrSuccess
    }
    pub fn is_ready(&self) -> bool {
        self.current >= self.max
    }
}
