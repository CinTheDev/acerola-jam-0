use bevy::prelude::*;

use crate::player::{collision::SphereCollider, items::{ItemDrop, ItemDropBundle, ItemId}};

pub mod rotate_button;

use rotate_button::{RotateButton, check_button_solution};

#[derive(Bundle)]
pub struct MasterTaskBundle {
    task: MasterTask,
}

#[derive(Bundle)]
pub struct CopperTaskBundle {
    item_drop: ItemDropBundle,
    task: CopperTask,
}

#[derive(Bundle)]
pub struct RotateButtonsTaskBundle {
    task: RotateButtonsTask,
}

#[derive(Component)]
pub struct MasterTask {
    is_all_done: bool,
}

#[derive(Component)]
pub struct CopperTask {
    is_done: bool,
}

#[derive(Component)]
pub struct RotateButtonsTask {
    is_active: bool,
    is_done: bool,
}

pub fn check_buttons_solution(
    q_buttons: Query<&RotateButton>,
    mut q_task: Query<&mut RotateButtonsTask>,
    mut q_master: Query<&mut MasterTask>,
) {
    let mut task = q_task.single_mut();
    let mut master = q_master.single_mut();

    if ! task.is_active || task.is_done { return }

    let solution_correct = check_button_solution(q_buttons);

    if ! solution_correct { return }

    task.is_done = true;
    task.is_active = false;

    master.is_all_done = true;

    info!("Particle accelerator task done");
}

fn enable_buttons(query: Query<&mut SphereCollider, With<RotateButton>>) {
    rotate_button::activate_buttons(query);
}

pub fn check_coppertask(
    mut q_task: Query<(&mut CopperTask, &ItemDrop)>,
    q_buttons: Query<&mut SphereCollider, With<RotateButton>>,
) {
    let mut task = q_task.single_mut();

    let task_prop = task.0.as_mut();
    let task_drop = task.1;

    if task_prop.is_done { return }

    if ! task_drop.is_dropped { return }

    task_prop.is_done = true;
    enable_buttons(q_buttons);
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
