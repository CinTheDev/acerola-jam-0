// Structure and general implementation of tasks
use bevy::prelude::*;

pub mod test_task;

#[derive(Component)]
pub struct TaskManager {
    task_index: usize,
    tasks: [Task],
}

trait Task {
    // Task has 3 behaviours: start task, do (update) task, finish task
    // These must be implemented per task
    fn start_task(&self);
    fn check_task(&self);
    fn finish_task(&self);

    // If task finishes, next task will be started
}
