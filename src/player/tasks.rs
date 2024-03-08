// Structure and general implementation of tasks
use bevy::prelude::*;

pub mod q_t_de;
pub mod alloy_machine;
pub mod computer;
pub mod particle_accelerator;

pub fn instance_tasks(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(q_t_de::instance_dark_matter());

    commands.spawn(alloy_machine::instance_master());
    commands.spawn(alloy_machine::instance_lead());
    commands.spawn(alloy_machine::instance_ironblock());
    commands.spawn(alloy_machine::instance_ironhammer());
    commands.spawn(alloy_machine::instance_ironscrewdriver());
    commands.spawn(alloy_machine::instance_ironphone());

    commands.spawn(computer::instance_computer());

    commands.spawn(particle_accelerator::instance_master());
    commands.spawn(particle_accelerator::instance_copper());
    commands.spawn(particle_accelerator::instance_buttons());

    particle_accelerator::rotate_button::spawn_buttons(commands, asset_server);
}

trait ItemDropTask {
    fn is_done(&self) -> bool;
    fn set_done(&mut self, val: bool);
}
