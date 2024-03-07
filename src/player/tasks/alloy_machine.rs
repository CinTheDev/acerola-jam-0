use bevy::prelude::*;

use crate::player::{collision::SphereCollider, items::{ItemDrop, ItemDropBundle, ItemId}};
use super::ItemDropTask;

#[derive(Bundle)]
pub struct AlloyMachineTaskBundle {
    master_task: MasterTask,
}

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
pub struct MasterTask {
    is_all_done: bool,
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

fn finish_master_task(mut commands: Commands, task_master: &mut MasterTask) {
    info!("Alloy machine task has been finished.");

    // TODO: Spawn exotic matter
}

pub fn check_if_finished(
    commands: Commands,
    mut q_task_master: Query<&mut MasterTask>,
    q_task_lead: Query<(&mut LeadTask, &ItemDrop)>,
    q_task_block: Query<(&mut IronBlockTask, &ItemDrop)>,
    q_task_hammer: Query<(&mut IronHammerTask, &ItemDrop)>,
    q_task_screwdriver: Query<(&mut IronScrewdriverTask, &ItemDrop)>,
    q_task_phone: Query<(&mut IronPhoneTask, &ItemDrop)>,
) {
    let mut task_master = q_task_master.single_mut();

    if task_master.is_all_done { return }

    let all_tasks_finished = 
        check_task(q_task_lead) &&
        check_task(q_task_block) &&
        check_task(q_task_hammer) &&
        check_task(q_task_screwdriver) &&
        check_task(q_task_phone);
    
    task_master.is_all_done = all_tasks_finished;
    finish_master_task(commands, task_master.as_mut());
}

fn check_task<T: bevy::prelude::Component + super::ItemDropTask>(mut q_task: Query<(&mut T, &ItemDrop)>) -> bool {
    let mut task_ref = q_task.single_mut();
    let task = task_ref.0.as_mut();
    let itemdrop = task_ref.1;

    if task.is_done() { return true }

    if ! itemdrop.is_dropped { return false }

    task.set_done(true);
    info!("Generic task done");

    return true;
}

pub fn instance_lead() -> LeadTaskBundle {
    LeadTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(8.0, 1.6, 2.84),
            collider: SphereCollider {
                radius: 0.5,
                enabled: true,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::Lead,
                activates_id: ItemId::None,
                is_dropped: false,
            }
        },
        task: LeadTask {
            is_done: false
        }
    }
}

pub fn instance_ironblock() -> IronBlockTaskBundle {
    IronBlockTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(8.0, 1.5, 0.3),
            collider: SphereCollider {
                radius: 0.5,
                enabled: false,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::IronBlock,
                activates_id: ItemId::None,
                is_dropped: false,
            }
        },
        task: IronBlockTask {
            is_done: false
        }
    }
}

pub fn instance_ironhammer() -> IronHammerTaskBundle {
    IronHammerTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(8.0, 1.5, 0.3),
            collider: SphereCollider {
                radius: 0.5,
                enabled: false,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::IronHammer,
                activates_id: ItemId::None,
                is_dropped: false,
            }
        },
        task: IronHammerTask {
            is_done: false
        }
    }
}

pub fn instance_ironscrewdriver() -> IronScrewdriverTaskBundle {
    IronScrewdriverTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(8.0, 1.5, 0.3),
            collider: SphereCollider {
                radius: 0.5,
                enabled: false,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::IronScrewdriver,
                activates_id: ItemId::None,
                is_dropped: false,
            }
        },
        task: IronScrewdriverTask {
            is_done: false
        }
    }
}

pub fn instance_ironphone() -> IronPhoneTaskBundle {
    IronPhoneTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(8.0, 1.5, 0.3),
            collider: SphereCollider {
                radius: 0.5,
                enabled: false,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::IronPhone,
                activates_id: ItemId::None,
                is_dropped: false,
            }
        },
        task: IronPhoneTask {
            is_done: false
        }
    }
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
