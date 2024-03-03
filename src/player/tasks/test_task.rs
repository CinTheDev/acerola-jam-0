// Task specific implementations

use bevy::prelude::*;
use super::Task;

pub struct test_task {
    some_val: i32,
}

impl test_task {
    pub fn new(val: i32) -> Self {
        Self {
            some_val: val,
        }
    }
}

impl Task for test_task {
    fn start_task(&self) {
        info!("Starting task: {}", self.some_val);
    }

    fn check_task(&mut self) {
        self.some_val += 1;
    }

    fn finish_task(&self) {
        info!("Finished task: {}", self.some_val);
    }
}
