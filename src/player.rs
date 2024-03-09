use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::PI;

const PLAYER_MAX_ROTATION: f32 =  PI / 2.0 - 0.05;
const PLAYER_MIN_ROTATION: f32 = -PI / 2.0 + 0.05;

pub mod collision;
use collision::{PlaneCollider, SphereCollider};

pub mod items;
use items::ItemId;

pub mod tasks;

pub fn instance_player(commands: &mut Commands) {
    commands.spawn(PlayerBundle {
        player: Player {
            speed: 3.0,
            sensitivity: 0.001,
            rotation: Vec2::ZERO,
            item_id: ItemId::None,
            locked: false,
        },
        collider: SphereCollider {
            radius: 1.0,
            enabled: true,
        },
        camera: Camera3dBundle {
            transform: Transform::from_xyz(3.0, 1.5, 0.0),
            ..default()
        }
    });
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    collider: SphereCollider,
    camera: Camera3dBundle,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    sensitivity: f32,

    rotation: Vec2,
    pub item_id: ItemId,

    locked: bool,
}

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut q_player: Query<(&mut Player, &mut Transform, &SphereCollider)>,
    q_walls_collider: Query<(&PlaneCollider, &Transform), Without<Player>>,
) {
    // Variables setup
    let mut p = q_player.single_mut();
    let properties = p.0.as_mut();
    let transform_copy = p.1.as_ref().clone();
    let transform = p.1.as_mut();

    if properties.locked { return } // Don't move if player is locked
    
    // Input processing
    let dir = get_keyboard_input(&keyboard_input, &transform);
    let mouse_delta = get_mouse_input(&mut mouse_input);
    
    let vec_move = dir * properties.speed * time.delta_seconds();
    let trans_rot = mouse_delta * properties.sensitivity;
    
    // Collision checks
    let player_tuple = (p.2, &transform_copy);
    let vec_move_checked = check_player_collisions(player_tuple, q_walls_collider, vec_move);

    // Moving the player
    transform.translation += vec_move_checked;

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
    player: (&SphereCollider, &Transform),
    q_walls: Query<(&PlaneCollider, &Transform), Without<Player>>,
    player_velocity: Vec3,
) -> Vec3 {
    let p_sphere_col = player.0;
    let p_pos = player.1.translation.xz();

    let mut result = player_velocity;

    for _ in 0..2 {
        for wall in q_walls.iter() {
            let wall_properties = wall.0;
            let wall_pos = wall.1.translation.xz();
    
            let collision_result = collision::check_collision_dynamic(
                p_sphere_col,
                p_pos,
                wall_properties,
                wall_pos,
                result.xz()
            );
    
            if collision_result.0 {
                result = Vec3::new(collision_result.1.x, 0.0, collision_result.1.y);
            }
        }
    }

    return result;
}

pub fn raycast_items(
    q_player: Query<&Transform, With<Player>>,
    q_items: Query<(&Transform, &SphereCollider, &items::Item)>,
    q_drops: Query<(&Transform, &SphereCollider, &items::ItemDrop)>,
    input: Res<Input<KeyCode>>,
    ev_pickup: EventWriter<items::PickupEvent>,
    ev_drop: EventWriter<items::DropEvent>,
    ev_cancel: EventWriter<items::DropCancelEvent>,
) {
    let player_trans = q_player.single();

    let ray = player_trans.forward() * 5.0;
    let raycast_item = collision::raycast(player_trans.translation, ray, q_items.iter());
    let raycast_drop = collision::raycast(player_trans.translation, ray, q_drops.iter());

    //info!("Raycast item result: {}", raycast_item.is_some());
    //info!("Raycast drop result: {}", raycast_drop.is_some());
    
    check_drop_item(&input, ev_cancel);
    check_raycast_item(raycast_item, &input, ev_pickup);
    check_raycast_itemdrop(raycast_drop, &input, ev_drop);
}

fn check_raycast_item<'a>(
    raycast_result: Option<&items::Item>,
    input: &Res<Input<KeyCode>>,
    mut ev_pickup: EventWriter<items::PickupEvent>,
) {
    if raycast_result.is_none() || !input.pressed(KeyCode::F) { return }

    let item = raycast_result.unwrap();
    ev_pickup.send(items::PickupEvent(item.id));
}

fn check_raycast_itemdrop<'a>(
    raycast_result: Option<&items::ItemDrop>,
    input: &Res<Input<KeyCode>>,
    mut ev_drop: EventWriter<items::DropEvent>,
) {
    if raycast_result.is_none() || !input.pressed(KeyCode::F) { return }

    let itemdrop = raycast_result.unwrap();
    ev_drop.send(items::DropEvent(itemdrop.accepts_id));
}

fn check_drop_item(
    input: &Res<Input<KeyCode>>,
    mut ev_cancel: EventWriter<items::DropCancelEvent>,
) {
    if input.just_pressed(KeyCode::X) {
        ev_cancel.send(items::DropCancelEvent());
    }
}
