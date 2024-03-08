use bevy::prelude::*;
use crate::player::{collision::SphereCollider, items::{ItemDrop, ItemDropBundle, ItemId}};
use super::particle_accelerator;

#[derive(Bundle)]
pub struct CleanDarkMatterBundle {
    pub item_drop: ItemDropBundle,
    pub task: CleanDarkMatterTask,
}

#[derive(Component)]
pub struct CleanDarkMatterTask {
    pub is_done: bool
}

pub fn check_all_tasks_finished(
    q_darkmatter: Query<&CleanDarkMatterTask>,
    // TODO: q_alloy:
    q_particle_accelerator: Query<&particle_accelerator::MasterTask>,
    // TODO: Final button
) {
    let task_darkmatter = q_darkmatter.single();
    let task_particle_accelerator = q_particle_accelerator.single();

    let all_done =
        task_darkmatter.is_done &&
        task_particle_accelerator.is_all_done;
    
    if ! all_done { return }

    info!("Yay we did it")
}

pub fn check_dark_matter_finished(mut q_task: Query<(&mut CleanDarkMatterTask, &ItemDrop)>) {
    let mut task_ref = q_task.single_mut();
    let task = task_ref.0.as_mut();
    let itemdrop = task_ref.1;

    if task.is_done { return }

    if ! itemdrop.is_dropped { return }

    task.is_done = true;
    info!("Dark matter task finished");
}

pub fn instance_dark_matter() -> CleanDarkMatterBundle {
    CleanDarkMatterBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(-9.0, 0.75, 8.0),
            collider: SphereCollider {
                radius: 1.0,
                enabled: true,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::DarkMatter,
                activates_id: ItemId::None,
                is_dropped: false,
            },
        },
        task: CleanDarkMatterTask {
            is_done: false,
        }
    }
}
