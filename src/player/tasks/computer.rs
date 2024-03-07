use bevy::prelude::*;

use crate::player::{collision::{raycast, SphereCollider}, Player};

const PASSWORD: &str = "abc";

#[derive(Bundle)]
pub struct ComputerTaskBundle {
    transform: Transform,
    collider: SphereCollider,
    task: ComputerTask,
}

#[derive(Component)]
pub struct ComputerTask {
    is_active: bool,
    is_finished: bool,
}

pub fn check_activation(
    q_player: Query<(&Player, &SphereCollider, &Transform)>,
    mut q_task: Query<(&Transform, &SphereCollider, &mut ComputerTask)>,
    input: Res<Input<KeyCode>>,
) {
    let player = q_player.single();
    let player_prop = player.0;
    let player_coll = player.1;
    let player_trans = player.2;

    let mut task = q_task.single_mut();
    let task_coll = task.1;
    
    if task.2.as_ref().is_active {
        // TODO: Implement
        return;
    }
    
    // Check for player interaction
    let raycast_result = raycast(
        player_trans.translation,
        player_trans.forward() * 5.0,
        std::iter::once((task.0, task.1, task.2.as_ref()))
    );
    
    let interaction = check_interaction(input) && raycast_result.is_some();
    
    if ! interaction { return }
    
    task.2.as_mut().is_active = true;
    info!("Computer has been activated");
}

fn check_interaction(input: Res<Input<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::F)
}

pub fn instance_computer() -> ComputerTaskBundle {
    ComputerTaskBundle {
        transform: Transform::from_xyz(1.2, 1.0, 5.5),
        collider: SphereCollider {
            radius: 0.35,
            enabled: true,
        },
        task: ComputerTask { 
            is_active: false,
            is_finished: false,
        }
    }
}
