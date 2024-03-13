use bevy::prelude::*;
use std::f32::consts::PI;
use rand;

use crate::player::{collision::{raycast_mut, SphereCollider}, Player};
use crate::RaycastCursor;
use super::ParticleAcceleratorFinished;

const BUTTON_ROTATION: f32 = 28.252 * (PI / 180.0);
const LERP_FACTOR: f32 = 0.25;

#[derive(Bundle)]
pub struct RotateButtonBundle {
    scene: SceneBundle,
    collider: SphereCollider,
    rotate_button: RotateButton,
    r_cursor: RaycastCursor,
    respawn: crate::Respawn,
}

#[derive(Component)]
pub struct RotateButton {
    active_position: Vec3,
    passive_position: Vec3,
    is_active: bool,
    rotation: u8,
    position_x: usize,
    position_y: usize,
    rotatable: bool,
}

pub fn check_button_solution(q_buttons: Query<&RotateButton>) -> bool {
    for button in q_buttons.iter() {
        let correct_rotation = BUTTON_ROT_SOLUTION[button.position_y][button.position_x];
        let button_type = BUTTON_TYPES[button.position_y][button.position_x];
        
        match button_type {
            // Rotation doesn't matter
            0 => { continue }

            // Straight pipe has rotational symmetries, requires special handling
            1 => {
                if button.rotation % 2 != correct_rotation {
                    return false;
                }
            }

            // No rotational symmetries
            2 | 3 => {
                if button.rotation != correct_rotation {
                    return false;
                }
            }

            _ => { panic!("Invalid type in solution"); }
        }
    }

    return true;
}

pub fn check_button_interaction(
    mut q_buttons: Query<(&Transform, &SphereCollider, &mut RotateButton)>,
    q_player: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if ! input.just_pressed(KeyCode::F) { return }

    let player = q_player.single();

    let result = raycast_mut(
        player.translation,
        player.forward() * 5.0,
        q_buttons.iter_mut()
    );

    if result.is_none() { return }

    let mut button = result.unwrap();
    button.rotation = (button.rotation + 1) % 4;
}

pub fn rotate_buttons(
    mut q_buttons: Query<(&mut Transform, &RotateButton)>
) {
    for button in q_buttons.iter_mut() {
        let mut button_trans = button.0;
        let button_prop = button.1;

        let button_total_rotation = Quat::from_rotation_x(BUTTON_ROTATION)
            * Quat::from_rotation_y(0.5 * PI * button_prop.rotation as f32);

        button_trans.rotation = button_trans.rotation.slerp(button_total_rotation, LERP_FACTOR);

        // Position
        let mut button_desired_pos = button_prop.passive_position;

        if button_prop.is_active {
            button_desired_pos = button_prop.active_position;
        }

        button_trans.translation = button_trans.translation.lerp(button_desired_pos, LERP_FACTOR);
    }
}

pub fn activate_buttons(mut query: Query<(&mut SphereCollider, &mut RotateButton)>) {
    for (mut collider, mut button) in query.iter_mut() {
        collider.enabled = button.rotatable;
        button.is_active = true;
    }
}

pub fn disable_buttons(
    mut ev_finished: EventReader<ParticleAcceleratorFinished>,
    mut q_buttons: Query<(&mut SphereCollider, &mut RotateButton)>,
) {
    for _ in ev_finished.read() {
        for (mut coll, mut button) in q_buttons.iter_mut() {
            coll.enabled = false;
            button.is_active = false;
        }
    }
}

pub fn spawn_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let first_transform = Transform::from_xyz(-0.65, 0.85, -8.8)
        .with_rotation(Quat::from_rotation_x(BUTTON_ROTATION));

    let dist_right: f32 = 0.228 * 1.14;
    let dist_down: f32 = 0.228 * 1.15;

    for x in 0..14 {
        for y in 0..4 {
            let active_trans = first_transform.with_translation(
                first_transform.translation
                + first_transform.right() * dist_right * x as f32
                + first_transform.back() * dist_down * y as f32
            );

            let start_trans = active_trans.with_translation(
                active_trans.translation + first_transform.down() * 0.05
            );

            let mut rotation = rand::random::<u8>() % 4;

            if BUTTON_ENABLED[y][x] == 0 {
                rotation = BUTTON_ROT_SOLUTION[y][x];
            }

            let scene_handle: Handle<Scene> = match BUTTON_TYPES[y][x] {
                0 => asset_server.load("items/rotate_button_1.glb#Scene0"),
                1 => asset_server.load("items/rotate_button_straight.glb#Scene0"),
                2 => asset_server.load("items/rotate_button_corner.glb#Scene0"),
                3 => asset_server.load("items/rotate_button_T.glb#Scene0"),

                _ => panic!("Button type {} out of defined bounds", BUTTON_TYPES[y][x]),
            };

            commands.spawn(RotateButtonBundle {
                scene: SceneBundle {
                    scene: scene_handle,
                    transform: start_trans,
                    ..default()
                },
                collider: SphereCollider {
                    radius: 0.1,
                    enabled: false,
                },
                rotate_button: RotateButton {
                    active_position: active_trans.translation,
                    passive_position: start_trans.translation,
                    is_active: false,
                    rotation,
                    position_x: x,
                    position_y: y,
                    rotatable: BUTTON_ENABLED[y][x] == 1,
                },
                r_cursor: RaycastCursor,
                respawn: crate::Respawn,
            });
        }
    }
}

const BUTTON_ENABLED: [[u8; 14]; 4] = [
    [0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
];

const BUTTON_TYPES: [[u8; 14]; 4] = [
    [1, 1, 2, 0, 0, 0, 0, 0, 0, 2, 1, 1, 2, 2],
    [1, 2, 2, 1, 1, 3, 1, 3, 1, 3, 2, 2, 3, 1],
    [2, 3, 1, 2, 0, 3, 2, 2, 1, 3, 3, 2, 1, 3],
    [3, 2, 0, 2, 1, 2, 2, 1, 1, 3, 1, 1, 3, 3],
];

const BUTTON_ROT_SOLUTION: [[u8; 14]; 4] = [
    [0, 0, 3, 9, 9, 9, 9, 9, 9, 0, 0, 0, 3, 0],
    [0, 3, 1, 0, 0, 3, 0, 3, 0, 1, 3, 0, 2, 1],
    [3, 0, 0, 3, 9, 0, 3, 1, 0, 3, 1, 2, 1, 0],
    [1, 2, 9, 1, 0, 2, 1, 0, 0, 1, 0, 0, 1, 1],
];
