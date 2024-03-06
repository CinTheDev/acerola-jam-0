// Structure and general implementation of tasks
use bevy::prelude::*;

pub mod clean_dark_matter;

pub fn instance_tasks(mut commands: Commands) {
    commands.spawn(clean_dark_matter::instance());
}
