use std::f32::consts::PI;

use bevy::prelude::*;

use crate::player::{collision::SphereCollider, items::{ItemDrop, ItemDropBundle, ItemId}};
use crate::RaycastCursor;
use crate::sound::{PlaySpatialSoundEvent, SoundID};

pub mod rotate_button;

use rotate_button::{RotateButton, check_button_solution};

#[derive(Event)]
pub struct ParticleAcceleratorFinished();

#[derive(Bundle)]
pub struct MasterTaskBundle {
    task: MasterTask,
    respawn: crate::Respawn,
}

#[derive(Bundle)]
pub struct CopperTaskBundle {
    item_drop: ItemDropBundle,
    task: CopperTask,
}

#[derive(Bundle)]
pub struct RotateButtonsTaskBundle {
    task: RotateButtonsTask,
    respawn: crate::Respawn,
}

#[derive(Component)]
pub struct MasterTask {
    pub is_all_done: bool,
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
    mut event: EventWriter<ParticleAcceleratorFinished>,
    mut ev_sound: EventWriter<PlaySpatialSoundEvent>,
) {
    let mut task = q_task.single_mut();
    let mut master = q_master.single_mut();

    if ! task.is_active || task.is_done { return }

    let solution_correct = check_button_solution(q_buttons);

    if ! solution_correct { return }

    task.is_done = true;
    task.is_active = false;

    master.is_all_done = true;

    event.send(ParticleAcceleratorFinished());
    ev_sound.send(PlaySpatialSoundEvent(SoundID::ParticleAccelerator, Vec3::new(0.0, 2.0, 21.0)));
    info!("Particle accelerator task done");
}

fn enable_buttons(query: Query<(&mut SphereCollider, &mut RotateButton)>, task: &mut RotateButtonsTask) {
    rotate_button::activate_buttons(query);
    task.is_active = true;
}

pub fn check_coppertask(
    mut q_coppertask: Query<(&mut CopperTask, &ItemDrop)>,
    mut q_buttontask: Query<&mut RotateButtonsTask>,
    q_buttons: Query<(&mut SphereCollider, &mut RotateButton)>,
) {
    let mut coppertask = q_coppertask.single_mut();
    let mut buttontask = q_buttontask.single_mut();

    let coppertask_prop = coppertask.0.as_mut();
    let coppertask_drop = coppertask.1;

    if coppertask_prop.is_done { return }

    if ! coppertask_drop.is_dropped { return }

    coppertask_prop.is_done = true;
    
    enable_buttons(q_buttons, buttontask.as_mut());
}

pub fn instance_master() -> MasterTaskBundle {
    MasterTaskBundle {
        task: MasterTask {
            is_all_done: false,
        },
        respawn: crate::Respawn,
    }
}

pub fn instance_copper() -> CopperTaskBundle {
    CopperTaskBundle {
        item_drop: ItemDropBundle {
            transform: Transform::from_xyz(-0.6, 0.5, -9.4281)
                .with_rotation(Quat::from_rotation_y(PI / -2.0)),
            collider: SphereCollider {
                radius: 0.3,
                enabled: true,
            },
            item_drop: ItemDrop {
                accepts_id: ItemId::CopperFuel,
                is_dropped: false,
            },
            r_cursor: RaycastCursor,
            respawn: crate::Respawn,
        },
        task: CopperTask {
            is_done: false,
        },
    }
}

pub fn instance_buttons() -> RotateButtonsTaskBundle {
    RotateButtonsTaskBundle {
        task: RotateButtonsTask {
            is_active: false,
            is_done: false,
        },
        respawn: crate::Respawn,
    }
}
