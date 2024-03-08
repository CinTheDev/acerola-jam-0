use bevy::prelude::*;
use std::f32::consts::PI;
use rand;

use crate::player::{collision::{raycast_mut, SphereCollider}, Player};

const BUTTON_ROTATION: f32 = 28.252 * (PI / 180.0);
const LERP_FACTOR: f32 = 0.25;

#[derive(Bundle)]
pub struct RotateButtonBundle {
    scene: SceneBundle,
    collider: SphereCollider,
    rotate_button: RotateButton,
}

#[derive(Component)]
pub struct RotateButton {
    rotation: u8,
    position_x: usize,
    position_y: usize,
}

pub fn check_button_solution(
    q_buttons: Query<&RotateButton>
) {
    for button in q_buttons.iter() {
        let correct_rotation = BUTTON_ROT_SOLUTION[button.position_y][button.position_x];

        if button.rotation % 4 != correct_rotation {
            info!("Puzzle is no");
            return;
        }
    }

    info!("Puzzle is Yeah");
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
    button.rotation += 1;
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
    }
}

pub fn spawn_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let first_transform = Transform::from_xyz(-0.609, 0.85, -8.736)
        .with_rotation(Quat::from_rotation_x(BUTTON_ROTATION));

    let dist_right: f32 = 0.228 * 1.14;
    let dist_down: f32 = 0.228 * 1.15;

    for x in 0..14 {
        for y in 0..4 {
            let trans = first_transform.with_translation(
                first_transform.translation
                + first_transform.right() * dist_right * x as f32
                + first_transform.back() * dist_down * y as f32
            );

            //let rotation = rand::random::<u8>() % 4;

            let scene_handle: Handle<Scene> = match BUTTON_TYPES[y][x] {
                0 => asset_server.load("items/rotate_button_1.glb#Scene0"),
                1 => asset_server.load("items/rotate_button_straight.glb#Scene0"),
                2 => asset_server.load("items/rotate_button_corner.glb#Scene0"),
                3 => asset_server.load("items/rotate_button_T.glb#Scene0"),

                _ => panic!("Button type {} out of defined bounds", BUTTON_TYPES[x][y]),
            };

            commands.spawn(RotateButtonBundle {
                scene: SceneBundle {
                    scene: scene_handle,
                    transform: trans,
                    ..default()
                },
                collider: SphereCollider {
                    radius: 0.1,
                    enabled: true,
                },
                rotate_button: RotateButton {
                    //rotation,
                    rotation: 0,
                    position_x: x,
                    position_y: y,
                },
            });
        }
    }
}

const BUTTON_TYPES: [[u8; 14]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
];

const BUTTON_ROT_SOLUTION: [[u8; 14]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
];
