// Structure and general implementation of tasks
use bevy::prelude::*;

const TASK_COUNT: usize = 1;

pub mod test_task;

#[derive(Component)]
pub struct TaskManager {
    task_index: usize,
    tasks: [Box<dyn Task + Send + Sync>; TASK_COUNT],
}

trait Task {
    // Task has 3 behaviours: start task, do (update) task, finish task
    // These must be implemented per task
    fn start_task(&self);
    fn check_task(&mut self);
    fn finish_task(&self);

    // If task finishes, next task will be started
}

pub fn instance_tasks() -> TaskManager {
    let test_task = test_task::test_task::new(2);
    let test_task_box = Box::new(test_task);

    TaskManager {
        task_index: 0,
        tasks: [test_task_box],
    }
}
