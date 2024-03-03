// Task specific implementations

use bevy::prelude::*;
use super::Task;

// For testing:
// Start the task by bringing the pink thing to the other thing
// Finish task by pressing F on Keyboard

pub struct TestTask {
    some_val: i32,
}

impl TestTask {
    pub fn new(val: i32) -> Self {
        Self {
            some_val: val,
        }
    }
}

impl Task for TestTask {
    fn check_start(&mut self, input: Res<Input<KeyCode>>) -> bool {
        input.pressed(KeyCode::F)
    }

    fn start_task(&self) {
        info!("Starting task: {}", self.some_val);
    }

    fn check_task(&mut self) -> bool {
        self.some_val += 1;

        info!("Doing task: {}", self.some_val);

        // Finish task when
        self.some_val > 99
    }

    fn finish_task(&self) {
        info!("Finished task: {}", self.some_val);
    }
}
