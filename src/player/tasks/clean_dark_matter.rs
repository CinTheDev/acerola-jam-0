use bevy::prelude::*;
use crate::player::{collision::SphereCollider, items::{ItemDrop, ItemDropBundle, ItemId}};

#[derive(Bundle)]
pub struct CleanDarkMatterBundle {
    pub item_drop: ItemDropBundle,
    pub task: CleanDarkMatterTask,
}

#[derive(Component)]
pub struct CleanDarkMatterTask {
    pub is_done: bool
}

pub fn check_if_finished(mut q_task: Query<(&mut CleanDarkMatterTask, &ItemDrop)>) {
    let mut task_ref = q_task.single_mut();
    let task = task_ref.0.as_mut();
    let itemdrop = task_ref.1;

    if task.is_done { return }

    if ! itemdrop.is_dropped { return }

    task.is_done = true;
    info!("Dark matter task finished");
}

pub fn instance() -> CleanDarkMatterBundle {
    CleanDarkMatterBundle {
        item_drop: ItemDropBundle {
            collider: SphereCollider {
                radius: 1.0
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
