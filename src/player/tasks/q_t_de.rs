use bevy::prelude::*;
use crate::player::{collision::{raycast, SphereCollider}, items::{ItemDrop, ItemDropBundle, ItemId}, Player};
use super::computer;
use super::alloy_machine;
use super::particle_accelerator;

#[derive(Bundle)]
pub struct CleanDarkMatterBundle {
    pub item_drop: ItemDropBundle,
    pub task: CleanDarkMatterTask,
}

#[derive(Bundle)]
pub struct FinalButtonTaskBundle {
    transform: Transform,
    collider: SphereCollider,
    task: FinalButtonTask,
}

#[derive(Component)]
pub struct CleanDarkMatterTask {
    pub is_done: bool
}

#[derive(Component)]
pub struct FinalButtonTask {
    is_done: bool,
}

pub fn check_all_tasks_finished(
    q_darkmatter: Query<&CleanDarkMatterTask>,
    q_computer: Query<&computer::ComputerTask>,
    q_alloy: Query<&alloy_machine::MasterTask>,
    q_particle_accelerator: Query<&particle_accelerator::MasterTask>,
    mut q_finalbutton: Query<(&FinalButtonTask, &mut SphereCollider)>
) {
    let task_darkmatter = q_darkmatter.single();
    let task_computer = q_computer.single();
    let task_alloy = q_alloy.single();
    let task_particle_accelerator = q_particle_accelerator.single();
    let mut task_finalbutton = q_finalbutton.single_mut();

    let tasks_done =
        task_darkmatter.is_done &&
        task_computer.is_finished &&
        task_alloy.is_all_done &&
        task_particle_accelerator.is_all_done;
    
    if ! tasks_done { return }

    activate_final_button(task_finalbutton.1.as_mut());

    if ! task_finalbutton.0.is_done { return }

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

pub fn check_final_button_input(
    mut q_finalbutton: Query<(&Transform, &SphereCollider, &mut FinalButtonTask)>,
    q_player: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if ! input.just_pressed(KeyCode::F) || q_finalbutton.single().2.is_done { return }

    let player = q_player.single();

    let raycast_hit = raycast(
        player.translation,
        player.forward() * 5.0,
        q_finalbutton.iter(),
    ).is_some();

    if ! raycast_hit { return }

    let mut finalbutton = q_finalbutton.single_mut().2;
    finalbutton.is_done = true;
}

fn activate_final_button(collider: &mut SphereCollider) {
    collider.enabled = true;
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

pub fn instance_finalbutton() -> FinalButtonTaskBundle {
    FinalButtonTaskBundle {
        transform: Transform::from_xyz(-2.5, 0.75, 2.5),
        collider: SphereCollider {
            radius: 0.1,
            enabled: false,
        },
        task: FinalButtonTask {
            is_done: false,
        }
    }
}
