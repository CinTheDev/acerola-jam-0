use bevy::prelude::*;
use super::{collision::SphereCollider, Item, ItemBundle, ItemId};

pub fn spawn_all_items(commands: &mut Commands, asset_server: Res<AssetServer>) {
    spawn_dark_matter(commands, asset_server);
}

fn spawn_dark_matter(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ItemBundle {
        scene: SceneBundle {
            scene: asset_server.load("items/dark_matter.glb#Scene0"),
            ..default()
        },
        collider: SphereCollider {
            radius: 0.25,
        },
        item: Item {
            id: ItemId::DarkMatter,
            pickup: true,
            desired_transform: Transform::IDENTITY,
            lerp_active: false,
        }
    });
}
