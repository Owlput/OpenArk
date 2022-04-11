use bevy::prelude::*;

use crate::{
    general_components::counters::TimeBasedCounter,
    resources::speed_modifier::GameSpeedModifier,
};

pub fn skill_tick_sec(
    time: Res<Time>,
    multiplier: Res<GameSpeedModifier>,
    mut query: Query<&mut TimeBasedCounter>,
) {
    for mut counter in query.iter_mut() {
        if !counter.is_ready() {
            counter.incr(time.delta_seconds_f64() * multiplier.get());
        }
    }
}
