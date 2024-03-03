// Structure and general implementation of tasks
use bevy::prelude::*;

use super::{collision, items::{ItemDrop, ItemDropBundle, ItemId}};

const TASK_COUNT: usize = 1;

pub mod test_task;

#[derive(Component)]
pub struct TaskManager {
    task_index: usize,
    tasks: [Box<dyn Task + Send + Sync>; TASK_COUNT],
    task_active: bool,
    all_done: bool,
}

pub fn task_manager(
    mut query: Query<&mut TaskManager>,
    data_check: Res<Input<KeyCode>>
) {
    let mut q_task_manager = query.single_mut();
    let task_manager = q_task_manager.as_mut();

    if task_manager.all_done {
        return;
    }

    if ! task_manager.task_active {
        task_manager.check_start();
        return;
    }

    // If a task is active
    task_manager.check_task(data_check);
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

    fn check_task(&mut self, data_check_task: Res<Input<KeyCode>>) {
        let current_task = self.tasks[self.task_index].as_mut();
        let is_task_done = current_task.check_task(data_check_task);

        if ! is_task_done {
            return;
        }

        current_task.finish_task();

        self.task_active = false;
        self.task_index += 1;

        self.check_all_done();
    }

    fn check_all_done(&mut self) {
        if self.task_index >= TASK_COUNT {
            self.all_done = true;
            info!("All tasks are finished.");
        }
    }
}

trait Task {
    fn check_start(&mut self) -> bool;
    fn start_task(&self);
    fn check_task(&mut self, input: Res<Input<KeyCode>>) -> bool;
    fn finish_task(&self);

    // If task finishes, next task will be started
}

pub fn instance_tasks(
    mut commands: Commands
) {
    let test_task = test_task::TestTask::new(ItemDropBundle {
        collider: collision::SphereCollider {
            radius: 0.1
        },
        item_drop: ItemDrop {
            accepts_id: ItemId::Something,
            activates_id: ItemId::SomethingElse,
            is_dropped: false,
        }
    });
    let test_task_box = Box::new(test_task);

    commands.spawn(TaskManager {
        task_index: 0,
        tasks: [test_task_box],
        task_active: false,
        all_done: false,
    });
}
