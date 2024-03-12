use bevy::prelude::*;

use crate::{player::{collision::SphereCollider, items::{Item, ItemDrop, ItemDropBundle, ItemId}}, RaycastCursor};
use super::ItemDropTask;
use crate::sound::{PlaySoundEvent, SoundID};

#[derive(Event)]
pub struct AlloyCreationFinshed();

#[derive(Event)]
pub struct AlloyPlacementFinished();

#[derive(Bundle)]
pub struct MasterTaskBundle {
    task: MasterTask,
    respawn: crate::Respawn,
}

#[derive(Bundle)]
pub struct AlloyTaskBundle {
    item_drop: ItemDropBundle,
    task: AlloyTask,
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
    pub is_all_done: bool,
}

#[derive(Component)]
pub struct AlloyTask {
    is_done: bool,
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

fn output_alloy(mut items: Query<(&mut Visibility, &Item, &mut SphereCollider)>) {
    for mut i in items.iter_mut() {
        let item_id = i.1.id;

        if item_id != ItemId::ExoticAlloy { continue }

        let vis = i.0.as_mut();
        let coll = i.2.as_mut();
        *vis = Visibility::Inherited;
        coll.enabled = true;

    }
}

pub fn check_alloy_finished(
    mut q_master: Query<&mut MasterTask>,
    q_task: Query<&ItemDrop, With<AlloyTask>>,
    mut event: EventWriter<AlloyPlacementFinished>,
) {
    let mut task_master = q_master.single_mut();
    let task = q_task.single();

    if task_master.is_all_done { return }

    if ! task.is_dropped { return }

    task_master.is_all_done = true;
    event.send(AlloyPlacementFinished());
    info!("Finished alloy machine tasks");
}

pub fn check_if_finished(
    mut q_task_alloy: Query<&mut AlloyTask>,
    q_task_lead: Query<(&mut LeadTask, &ItemDrop)>,
    q_task_block: Query<(&mut IronBlockTask, &ItemDrop)>,
    q_task_hammer: Query<(&mut IronHammerTask, &ItemDrop)>,
    q_task_screwdriver: Query<(&mut IronScrewdriverTask, &ItemDrop)>,
    q_task_phone: Query<(&mut IronPhoneTask, &ItemDrop)>,
    q_items: Query<(&mut Visibility, &Item, &mut SphereCollider)>,
    mut event: EventWriter<AlloyCreationFinshed>,
    mut ev_sound: EventWriter<PlaySoundEvent>,
) {
    let mut alloy_task = q_task_alloy.single_mut();

    if alloy_task.is_done { return }

    let all_tasks_finished = 
        check_task(q_task_lead) &&
        check_task(q_task_block) &&
        check_task(q_task_hammer) &&
        check_task(q_task_screwdriver) &&
        check_task(q_task_phone);

    if all_tasks_finished {
        output_alloy(q_items);
        alloy_task.is_done = true;
        event.send(AlloyCreationFinshed());
        ev_sound.send(PlaySoundEvent(SoundID::AlloyMachine));
    }
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

pub fn instance_master() -> MasterTaskBundle {
    MasterTaskBundle {
        task: MasterTask {
            is_all_done: false,
        },
        respawn: crate::Respawn,
    }
}

pub fn instance_alloy() -> AlloyTaskBundle {
    AlloyTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(-2.691, 1.108, 0.761),
            collider: SphereCollider {
                radius: 0.25,
                enabled: true,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::ExoticAlloy,
                is_dropped: false
            },
            r_cursor: RaycastCursor,
            respawn: crate::Respawn,
        },
        task: AlloyTask {
            is_done: false,
        },
    }
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
                is_dropped: false,
            },
            r_cursor: RaycastCursor,
            respawn: crate::Respawn,
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
                is_dropped: false,
            },
            r_cursor: RaycastCursor,
            respawn: crate::Respawn,
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
                is_dropped: false,
            },
            r_cursor: RaycastCursor,
            respawn: crate::Respawn,
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
                is_dropped: false,
            },
            r_cursor: RaycastCursor,
            respawn: crate::Respawn,
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
                is_dropped: false,
            },
            r_cursor: RaycastCursor,
            respawn: crate::Respawn,
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
