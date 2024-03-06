use bevy::prelude::*;

use crate::player::items::{ItemDrop, ItemDropBundle};
use super::ItemDropTask;

#[derive(Bundle)]
pub struct LeadTaskBundle {
    item_drop: ItemDropBundle,
    task: LeadTask,
}

#[derive(Bundle)]
pub struct IronBlockTaskBundle {
    item_drop: ItemDropBundle,
    task: IronBlockTask,
}

#[derive(Bundle)]
pub struct IronHammerTaskBundle {
    item_drop: ItemDropBundle,
    task: IronHammerTask,
}

#[derive(Bundle)]
pub struct IronScrewdriverTaskBundle {
    item_drop: ItemDropBundle,
    task: IronScrewdriverTask,
}

#[derive(Bundle)]
pub struct IronPhoneTaskBundle {
    item_drop: ItemDropBundle,
    task: IronPhoneTask,
}

#[derive(Component)]
pub struct LeadTask {
    is_done: bool,
}

#[derive(Component)]
pub struct IronBlockTask {
    is_done: bool,
}

#[derive(Component)]
pub struct IronHammerTask {
    is_done: bool,
}

#[derive(Component)]
pub struct IronScrewdriverTask {
    is_done: bool,
}

#[derive(Component)]
pub struct IronPhoneTask {
    is_done: bool,
}

pub fn check_if_finished(
    q_task_lead: Query<(&mut LeadTask, &ItemDrop)>,
    q_task_block: Query<(&mut IronBlockTask, &ItemDrop)>,
    q_task_hammer: Query<(&mut IronHammerTask, &ItemDrop)>,
    q_task_screwdriver: Query<(&mut IronScrewdriverTask, &ItemDrop)>,
    q_task_phone: Query<(&mut IronPhoneTask, &ItemDrop)>,
) {
    check_task(q_task_lead);
    check_task(q_task_block);
    check_task(q_task_hammer);
    check_task(q_task_screwdriver);
    check_task(q_task_phone);
}

fn check_task<T: bevy::prelude::Component + super::ItemDropTask>(mut q_task: Query<(&mut T, &ItemDrop)>) {
    let mut task_ref = q_task.single_mut();
    let task = task_ref.0.as_mut();
    let itemdrop = task_ref.1;

    if task.is_done() { return }

    if ! itemdrop.is_dropped { return }

    task.set_done(true);
    info!("Generic task done");
}

impl ItemDropTask for LeadTask {
    fn is_done(&self) -> bool {
        return self.is_done;
    }
    fn set_done(&mut self, val: bool) {
        self.is_done = val;
    }
}

// TODO: Generalize implementations somehow

impl ItemDropTask for IronBlockTask {
    fn is_done(&self) -> bool {
        return self.is_done;
    }
    fn set_done(&mut self, val: bool) {
        self.is_done = val;
    }
}

impl ItemDropTask for IronHammerTask {
    fn is_done(&self) -> bool {
        return self.is_done;
    }
    fn set_done(&mut self, val: bool) {
        self.is_done = val;
    }
}

impl ItemDropTask for IronScrewdriverTask {
    fn is_done(&self) -> bool {
        return self.is_done;
    }
    fn set_done(&mut self, val: bool) {
        self.is_done = val;
    }
}

impl ItemDropTask for IronPhoneTask {
    fn is_done(&self) -> bool {
        return self.is_done;
    }
    fn set_done(&mut self, val: bool) {
        self.is_done = val;
    }
}
