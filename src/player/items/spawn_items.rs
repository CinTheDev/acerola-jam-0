use bevy::prelude::*;
use super::{collision::SphereCollider, Item, ItemBundle, ItemId};

pub fn spawn_all_items(commands: &mut Commands, asset_server: Res<AssetServer>) {
    spawn_dark_matter(commands, asset_server);
}

fn spawn_dark_matter(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let transform = Transform::from_xyz(-1.97848, 1.02251, 0.2582)
    .with_rotation(Quat::from_rotation_y(265.91_f32.to_radians()));

    spawn_item(
        commands,
        asset_server,
        "items/dark_matter.glb#Scene0".to_owned(),
        transform,
        0.25,
        ItemId::DarkMatter
    )
}

fn spawn_item(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    asset_path: String,
    transform: Transform,
    radius: f32,
    id: ItemId,
) {
    commands.spawn(ItemBundle {
        scene: SceneBundle {
            scene: asset_server.load(asset_path),
            transform,
            ..default()
        },
        collider: SphereCollider {
            radius,
        },
        item: Item {
            id,
            pickup: true,
            desired_transform: transform,
            lerp_active: true,
        }
    });
}
