use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::PI;
const PLAYER_MAX_ROTATION: f32 = PI;
const PLAYER_MIN_ROTATION: f32 = -PI;

pub fn instance_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player {
            speed: 3.0,
            sensitivity: 0.001,
            rotation: Vec2::ZERO,
        },
        camera: Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        }
    });
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    camera: Camera3dBundle,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    sensitivity: f32,

    rotation: Vec2,
}

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut query: Query<(&mut Player, &mut Transform)>
) {
    let mut p = query.single_mut();
    let properties = p.0.as_mut();
    let transform = p.1.as_mut();

    let dir = get_keyboard_input(&keyboard_input, &transform);
    let mouse_delta = get_mouse_input(&mut mouse_input);

    let vec_move = dir * properties.speed * time.delta_seconds();
    let trans_rot = mouse_delta * properties.sensitivity;

    transform.translation += vec_move;

    properties.rotation += trans_rot;
    properties.rotation.y = properties.rotation.y
        .min(PLAYER_MAX_ROTATION)
        .max(PLAYER_MIN_ROTATION);

    transform.rotation = Quat::from_axis_angle(Vec3::Y, properties.rotation.x);
    transform.rotation *= Quat::from_axis_angle(transform.local_x(), properties.rotation.y);
}

fn get_keyboard_input(input: &Res<Input<KeyCode>>, player_trans: &Transform) -> Vec3 {
    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        dir += player_trans.forward();
    }
    if input.pressed(KeyCode::A) {
        dir += player_trans.left();
    }
    if input.pressed(KeyCode::S) {
        dir += player_trans.back();
    }
    if input.pressed(KeyCode::D) {
        dir += player_trans.right();
    }

    return dir.normalize_or_zero();
}

fn get_mouse_input(motion_evr: &mut EventReader<MouseMotion>) -> Vec2 {
    let mut mouse_delta = Vec2::ZERO;

    for ev in motion_evr.read() {
        mouse_delta += ev.delta;
    }

    return mouse_delta;
}
