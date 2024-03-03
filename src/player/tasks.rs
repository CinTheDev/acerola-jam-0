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

    if ! task_manager.task_active {
        task_manager.check_start();
        return;
    }

    // If a task is active
    task_manager.check_task();
}

impl TaskManager {
    fn check_start(&mut self) {
        let current_task = self.tasks[self.task_index].as_mut();
        let is_task_activated = current_task.check_start();

        if ! is_task_activated {
            return;
        }

        // If task has been initiated
        self.task_active = true;
        
        let current_task = self.tasks[self.task_index].as_mut();
        current_task.start_task();
    }

    fn check_task(&mut self) {
        let current_task = self.tasks[self.task_index].as_mut();
        let is_task_done = current_task.check_task();

        if ! is_task_done {
            return;
        }

        current_task.finish_task();

        self.task_active = false;
        self.task_index += 1;
    }
}

trait Task {
    fn check_start(&mut self) -> bool;
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
