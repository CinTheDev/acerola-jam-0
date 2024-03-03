// Structure and general implementation of tasks
use bevy::prelude::*;

const TASK_COUNT: usize = 1;

pub mod test_task;

#[derive(Component)]
pub struct TaskManager {
    task_index: usize,
    tasks: [Box<dyn Task + Send + Sync>; TASK_COUNT],
    task_active: bool,
}

pub fn task_manager(
    mut query: Query<&mut TaskManager>
) {
    let mut q_task_manager = query.single_mut();
    let task_manager = q_task_manager.as_mut();

    let current_task = task_manager.tasks[task_manager.task_index].as_mut();

    if ! task_manager.task_active {
        return;
    }

    // If a task is active

    let is_task_done = current_task.check_task();

    if ! is_task_done {
        return;
    }

    current_task.finish_task();
    
    task_manager.task_active = false;
    task_manager.task_index += 1;
}

impl TaskManager {
    fn start_next_task(&mut self) {
        self.task_active = true;
        
        let current_task = self.tasks[self.task_index].as_mut();
        current_task.start_task();
    }
}

trait Task {
    // Task has 3 behaviours: start task, do (update) task, finish task
    // These must be implemented per task
    fn start_task(&self);
    fn check_task(&mut self) -> bool;
    fn finish_task(&self);

    // If task finishes, next task will be started
}

pub fn instance_tasks(
    mut commands: Commands
) {
    let test_task = test_task::TestTask::new(2);
    let test_task_box = Box::new(test_task);

    commands.spawn(TaskManager {
        task_index: 0,
        tasks: [test_task_box],
        task_active: false,
    });
}
