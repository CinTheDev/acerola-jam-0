use bevy::prelude::*;

use crate::player::{collision::SphereCollider, items::{ItemDrop, ItemDropBundle, ItemId}};

pub mod rotate_button;

#[derive(Bundle)]
pub struct MasterTaskBundle {
    task: MasterTask,
}

#[derive(Bundle)]
pub struct CopperTaskBundle {
    item_drop: ItemDropBundle,
    task: CopperTask,
}

#[derive(Component)]
pub struct MasterTask {
    is_all_done: bool,
}

#[derive(Component)]
pub struct CopperTask {
    is_done: bool,
}

fn enable_buttons() {
    todo!();
}

pub fn check_coppertask(
    mut q_task: Query<(&mut CopperTask, &ItemDrop)>,
) {
    let mut task = q_task.single_mut();

    let task_prop = task.0.as_mut();
    let task_drop = task.1;

    if task_prop.is_done { return }

    if ! task_drop.is_dropped { return }

    task_prop.is_done = true;
    enable_buttons();
}

pub fn instance_master() -> MasterTaskBundle {
    MasterTaskBundle {
        task: MasterTask {
            is_all_done: false,
        }
    }
}

pub fn instance_copper() -> CopperTaskBundle {
    CopperTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(-0.6, 0.45, -9.45),
            collider: SphereCollider {
                radius: 0.3,
                enabled: true,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::CopperFuel,
                activates_id: ItemId::None,
                is_dropped: false,
            },
        },
        task: CopperTask {
            is_done: false,
        },
    }
}
