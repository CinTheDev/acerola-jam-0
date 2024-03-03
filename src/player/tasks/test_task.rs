// Task specific implementations

use bevy::prelude::*;
use crate::player::items::{ItemDrop, ItemDropBundle};

// For testing:
// Start the task by bringing the pink thing to the other thing
// Finish task by pressing F on Keyboard

#[derive(Bundle)]
pub struct TestTaskBundle {
    pub scene: SceneBundle,
    pub item_drop: ItemDropBundle,
    pub test_task: TestTask,
}

#[derive(Component)]
pub struct TestTask {
    pub is_active: bool,
}

pub fn check_if_dropped(
    mut query: Query<(&mut TestTask, &ItemDrop)>,
) {
    let mut q_test_task = query.single_mut();
    let test_task = q_test_task.0.as_mut();
    let item_drop = q_test_task.1;

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
