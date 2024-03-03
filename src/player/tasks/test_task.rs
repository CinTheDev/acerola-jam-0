// Task specific implementations

use bevy::prelude::*;
use crate::player::items::ItemDropBundle;

// For testing:
// Start the task by bringing the pink thing to the other thing
// Finish task by pressing F on Keyboard

#[derive(Bundle)]
pub struct TestTaskBundle {
    pub scene: SceneBundle,
    pub test_task: TestTask,
}

#[derive(Component)]
pub struct TestTask {
    pub item_drop: ItemDropBundle,

    is_active: bool,
}

impl TestTask {
    pub fn new(item_drop_bundle: ItemDropBundle) -> Self {
        Self {
            item_drop: item_drop_bundle,
            is_active: false,
        }
    }
}

pub fn check_if_dropped(
    mut query: Query<&mut TestTask>,
) {
    let mut q_test_task = query.single_mut();
    let test_task = q_test_task.as_mut();
    let item_drop = &test_task.item_drop.item_drop;

    if ! item_drop.is_dropped {
        return;
    }

    info!("Something");

    // Yay the thing has been dropped
    test_task.is_active = true;
}

pub fn do_task(
    mut q_task: Query<&mut TestTask>,
    input: Res<Input<KeyCode>>,
) {
    let mut q_test_task = q_task.single_mut();
    let test_task = q_test_task.as_mut();

    if ! test_task.is_active {
        return;
    }

    info!("Task active");

    if ! input.pressed(KeyCode::F) {
        return;
    }

    // Finish task
    // TODO
    info!("Task finished");
    test_task.is_active = false;
}

/*
impl Task for TestTask {
    fn check_start(&mut self) -> bool {
        return self.val;
    }

    fn start_task(&self) {
        info!("Starting task");
    }

    fn check_task(&mut self, input: Res<Input<KeyCode>>) -> bool {
        info!("Checking task");

        // Finish task when
        input.pressed(KeyCode::F)
    }

    fn finish_task(&self) {
        info!("Finished task");
    }
}
*/
