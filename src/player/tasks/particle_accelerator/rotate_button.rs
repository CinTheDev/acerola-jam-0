use bevy::prelude::*;

use crate::player::{collision::{raycast_mut, SphereCollider}, Player};

#[derive(Bundle)]
pub struct RotateButtonBundle {
    scene: SceneBundle,
    collider: SphereCollider,
    rotate_button: RotateButton,
}

#[derive(Component)]
pub struct RotateButton {
    rotation: u8,
}

pub fn rotate_buttons(
    mut q_buttons: Query<(&Transform, &SphereCollider, &mut RotateButton)>,
    q_player: Query<(&Transform), With<Player>>
) {
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

pub fn spawn_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let first_transform = Transform::from_xyz(-0.609, 0.85, -8.736)
        .with_rotation(Quat::from_rotation_x(28.252_f32.to_radians()));

    let dist_right: f32 = 0.228 * 1.14;
    let dist_down: f32 = 0.228 * 1.15;

    for x in 0..14 {
        for y in 0..4 {
            let trans = first_transform.with_translation(
                first_transform.translation
                + first_transform.right() * dist_right * x as f32
                + first_transform.back() * dist_down * y as f32
            );

            commands.spawn(RotateButtonBundle {
                scene: SceneBundle {
                    scene: asset_server.load("items/rotate_button_1.glb#Scene0"),
                    transform: trans,
                    ..default()
                },
                collider: SphereCollider {
                    radius: 0.1,
                    enabled: true,
                },
                rotate_button: RotateButton {
                    rotation: 0,
                },
            });
        }
    }
}
