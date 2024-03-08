// Structure and general implementation of tasks
use bevy::prelude::*;

pub mod clean_dark_matter;
pub mod alloy_machine;
pub mod computer;
pub mod particle_accelerator;

pub fn instance_tasks(mut commands: Commands) {
    commands.spawn(clean_dark_matter::instance());

    commands.spawn(alloy_machine::instance_master());
    commands.spawn(alloy_machine::instance_lead());
    commands.spawn(alloy_machine::instance_ironblock());
    commands.spawn(alloy_machine::instance_ironhammer());
    commands.spawn(alloy_machine::instance_ironscrewdriver());
    commands.spawn(alloy_machine::instance_ironphone());

    commands.spawn(computer::instance_computer());

    commands.spawn(particle_accelerator::instance_master());
    commands.spawn(particle_accelerator::instance_copper());
}

trait ItemDropTask {
    fn is_done(&self) -> bool;
    fn set_done(&mut self, val: bool);
}
