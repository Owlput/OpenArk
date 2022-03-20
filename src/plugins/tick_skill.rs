use bevy::prelude::*;

use crate::{
    systems::skill_counter::skill_tick_sec,
};

pub struct TickSkillPlugin;

impl Plugin for TickSkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(skill_tick_sec);
    }
}
