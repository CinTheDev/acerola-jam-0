// Task specific implementations

use bevy::prelude::*;
use super::Task;
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
    val: bool,
}

impl TestTask {
    pub fn new(item_drop_bundle: ItemDropBundle) -> Self {
        Self {
            item_drop: item_drop_bundle,
            val: false
        }
    }
}

pub fn check_if_dropped(
    mut query: Query<&mut TestTask>,
) {
    for mut thingy in query.iter_mut() {
        info!("Checking");
        let test_task = thingy.as_mut();
        let item_drop = &test_task.item_drop.item_drop;

        if ! item_drop.is_dropped {
            continue;
        }

        info!("Something");

        // Yay the thing has been dropped
        test_task.val = true;
    }
}

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
