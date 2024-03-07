use bevy::prelude::*;

use crate::player::{collision::{raycast, SphereCollider}, Player};

const PASSWORD: &str = "abc";
const LERP_FACTOR: f32 = 0.1;

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

    input: String,
}

pub fn check_activation(
    mut q_player: Query<(&mut Player, &mut SphereCollider, &mut Transform)>,
    mut q_task: Query<(&Transform, &SphereCollider, &mut ComputerTask), Without<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let mut player = q_player.single_mut();
    let player_prop = player.0.as_mut();
    let player_coll = player.1.as_mut();
    let player_trans = player.2.as_mut();

    let mut task = q_task.single_mut();
    
    if task.2.as_ref().is_active {
        // TODO: Implement
        lerp_camera(player_trans);
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
    
    info!("Computer has been activated");
    task.2.as_mut().is_active = true;
    lock_player(player_prop, player_coll);
}

pub fn input_from_keyboard(
    mut ev_char: EventReader<ReceivedCharacter>,
    mut q_task: Query<&mut ComputerTask>,
    input: Res<Input<KeyCode>>,
) {
    let mut task = q_task.single_mut();
    if ! task.is_active {
        ev_char.clear();
        return;
    }

    if input.just_pressed(KeyCode::Return) {
        if task.input == PASSWORD {
            // TODO: Task successful
        }
        else {
            clear_input(task.as_mut());
        }
    }

    if input.just_pressed(KeyCode::Back) {
        task.input.pop();
    }

    for c in ev_char.read() {
        if c.char.is_control() { continue }

        task.input.push(c.char);

        info!("Appended char. Now is: {}", task.input);
    }
}

fn lerp_camera(transform: &mut Transform) {
    let target = Transform::from_xyz(1.2, 1.05, 6.0).looking_to(-Vec3::Z, Vec3::Y);

    transform.translation = transform.translation.lerp(target.translation, LERP_FACTOR);
    transform.rotation = transform.rotation.slerp(target.rotation, LERP_FACTOR);
}

fn clear_input(task: &mut ComputerTask) {
    task.input.clear();
}

fn lock_player(player: &mut Player, collider: &mut SphereCollider) {
    player.locked = true;
    collider.enabled = false;
}

fn unlock_player(player: &mut Player, collider: &mut SphereCollider) {
    player.locked = false;
    collider.enabled = true;
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
            input: "".to_string(),
        }
    }
}
