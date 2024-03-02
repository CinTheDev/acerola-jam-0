use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::PI;

use self::collision::{BoxCollider, SphereCollider};
const PLAYER_MAX_ROTATION: f32 =  PI / 2.0 - 0.05;
const PLAYER_MIN_ROTATION: f32 = -PI / 2.0 + 0.05;

mod collision;

pub fn instance_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player {
            speed: 3.0,
            sensitivity: 0.001,
            rotation: Vec2::ZERO,
        },
        collider: collision::SphereCollider {
            position: Vec3::ZERO,
            radius: 1.0
        },
        camera: Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        }
    });

    // TODO: automate this (and move to other place)
    commands.spawn((
        Transform::from_xyz(3.0, 1.0, 3.0).with_scale(Vec3::new(2.0, 2.0, 2.0)),
        collision::BoxCollider {
            transform: Transform::from_scale(Vec3::new(2.0, 2.0, 2.0))
        }
    ));
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    collider: collision::SphereCollider,
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

    transform.rotation = Quat::IDENTITY;
    transform.rotate_y(properties.rotation.x * -1.0);
    transform.rotate_local_x(properties.rotation.y * -1.0);
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

    dir *= Vec3::new(1.0, 0.0, 1.0); // Disallow movement along y

    return dir.normalize_or_zero();
}

fn get_mouse_input(motion_evr: &mut EventReader<MouseMotion>) -> Vec2 {
    let mut mouse_delta = Vec2::ZERO;

    for ev in motion_evr.read() {
        mouse_delta += ev.delta;
    }

    return mouse_delta;
}

pub fn check_player_collisions(
    q_player: Query<(&SphereCollider, &Transform), With<Player>>,
    q_walls: Query<(&BoxCollider, &Transform)>
) {

}
