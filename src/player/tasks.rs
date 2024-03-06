// Structure and general implementation of tasks
use bevy::prelude::*;

pub mod clean_dark_matter;
pub mod alloy_machine;

pub fn instance_tasks(mut commands: Commands) {
    commands.spawn(clean_dark_matter::instance());
}

trait ItemDropTask {
    fn is_done(&self) -> bool;
    fn set_done(&mut self, val: bool);
}
