use bevy::prelude::*;

use crate::player::items::{ItemDrop, ItemDropBundle};
use super::ItemDropTask;

#[derive(Bundle)]
pub struct LeadTaskBundle {
    item_drop: ItemDropBundle,
    task: LeadTask,
}

#[derive(Component)]
pub struct LeadTask {
    is_done: bool,
}

impl ItemDropTask for LeadTask {
    fn is_done(&self) -> bool {
        return self.is_done;
    }
    fn set_done(&mut self, val: bool) {
        self.is_done = val;
    }
}

pub fn check_if_finished(
    q_task_lead: Query<(&mut LeadTask, &ItemDrop)>,
) {
    check_task(q_task_lead);
}

fn check_task<T: bevy::prelude::Component + super::ItemDropTask>(mut q_task_lead: Query<(&mut T, &ItemDrop)>) {
    let mut task_ref = q_task_lead.single_mut();
    let task = task_ref.0.as_mut();
    let itemdrop = task_ref.1;

    if task.is_done() { return }

    if ! itemdrop.is_dropped { return }

    task.set_done(true);
    info!("Generic task done");
}
