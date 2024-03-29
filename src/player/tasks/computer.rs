use bevy::prelude::*;

use crate::player::{collision::{raycast, SphereCollider}, Player};
use crate::RaycastCursor;
use crate::sound::{PlaySpatialSoundEvent, SoundID};

// Ambiguous letters like l, I, 1, or O, 0 have been swapped with other letters
const PASSWORD: &str = "tekMBtEYK4xXrULL7xY2NQILfu2cio";
const LERP_FACTOR: f32 = 0.1;

#[derive(Bundle)]
pub struct ComputerTaskBundle {
    transform: Transform,
    collider: SphereCollider,
    task: ComputerTask,
    r_cursor: RaycastCursor,
    respawn: crate::Respawn,
}

#[derive(Component)]
pub struct ComputerTask {
    pub is_active: bool,
    pub is_finished: bool,

    pub input: String,

    player_position: Transform,
}

#[derive(Event)]
pub struct SuccessEvent();

#[derive(Event)]
pub struct ErrorEvent;

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

    if task.2.is_finished { return }
    
    if task.2.is_active {
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
    
    task.2.as_mut().is_active = true;
    lock_player(player_prop, player_coll, *player_trans, task.2.as_mut());
}

pub fn input_from_keyboard(
    mut ev_char: EventReader<ReceivedCharacter>,
    mut ev_success: EventWriter<SuccessEvent>,
    mut ev_failure: EventWriter<ErrorEvent>,
    mut q_task: Query<&mut ComputerTask>,
    input: Res<Input<KeyCode>>,
    mut ev_sound: EventWriter<PlaySpatialSoundEvent>,
) {
    let sound_pos = Vec3::new(1.25, 1.0, 5.5);

    let mut task = q_task.single_mut();
    if ! task.is_active {
        ev_char.clear();
        return;
    }

    if input.just_pressed(KeyCode::Return) {
        if task.input == PASSWORD {
            ev_sound.send(PlaySpatialSoundEvent(SoundID::Keyboard, sound_pos));
            ev_success.send(SuccessEvent());
        }
        else {
            clear_input(task.as_mut());
            ev_failure.send(ErrorEvent);
            ev_sound.send(PlaySpatialSoundEvent(SoundID::ComputerDenied, sound_pos));
        }
    }

    if input.just_pressed(KeyCode::Back) {
        task.input.pop();
        ev_sound.send(PlaySpatialSoundEvent(SoundID::Keyboard, sound_pos));
    }

    for c in ev_char.read() {
        if c.char.is_control() { continue }

        task.input.push(c.char);

        ev_sound.send(PlaySpatialSoundEvent(SoundID::Keyboard, sound_pos));
    }
}

pub fn task_success(
    mut ev_success: EventReader<SuccessEvent>,
    mut q_player: Query<(&mut Player, &mut SphereCollider, &mut Transform)>,
    mut q_task: Query<&mut ComputerTask>,
) {
    for _ in ev_success.read() {
        let mut player = q_player.single_mut();
        let mut task = q_task.single_mut();

        task.is_finished = true;
        task.is_active = false;

        unlock_player(player.0.as_mut(), player.1.as_mut(), player.2.as_mut(), task.as_mut());
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

fn lock_player(player: &mut Player, collider: &mut SphereCollider, transform: Transform, task: &mut ComputerTask) {
    player.locked = true;
    collider.enabled = false;

    task.player_position = transform;
}

fn unlock_player(player: &mut Player, collider: &mut SphereCollider, transform: &mut Transform, task: &mut ComputerTask) {
    player.locked = false;
    collider.enabled = true;

    *transform = task.player_position;
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
            player_position: Transform::IDENTITY,
        },
        r_cursor: RaycastCursor,
        respawn: crate::Respawn,
    }
}
