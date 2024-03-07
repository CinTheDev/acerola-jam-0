use bevy::prelude::*;
use super::{collision::SphereCollider, Item, ItemBundle, ItemId};

pub fn spawn_all_items(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    spawn_dark_matter(commands, asset_server);
    spawn_lead(commands, asset_server);
    spawn_ironblock(commands, asset_server);
    spawn_ironhammer(commands, asset_server);
    spawn_ironscrewdriver(commands, asset_server);
    spawn_ironphone(commands, asset_server);
}

fn spawn_dark_matter(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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

fn spawn_lead(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let transform = Transform::from_xyz(8.5, 0.81, 4.75);

    spawn_item(
        commands,
        asset_server,
        "items/lead.glb#Scene0".to_owned(),
        transform,
        0.2,
        ItemId::Lead
    )
}

fn spawn_ironblock(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let transform = Transform::from_xyz(8.5, 0.81, 5.0);

    spawn_item(
        commands,
        asset_server,
        "items/iron_block.glb#Scene0".to_owned(),
        transform,
        0.2,
        ItemId::IronBlock
    )
}

fn spawn_ironhammer(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let transform = Transform::from_xyz(8.5, 0.81, 5.25);

    spawn_item(
        commands,
        asset_server,
        "items/iron_hammer.glb#Scene0".to_owned(),
        transform,
        0.2,
        ItemId::IronHammer
    )
}

fn spawn_ironscrewdriver(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let transform = Transform::from_xyz(8.5, 0.81, 5.5);

    spawn_item(
        commands,
        asset_server,
        "items/iron_screwdriver.glb#Scene0".to_owned(),
        transform,
        0.2,
        ItemId::IronScrewdriver
    )
}

fn spawn_ironphone(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let transform = Transform::from_xyz(8.5, 0.81, 5.75);

    spawn_item(
        commands,
        asset_server,
        "items/iron_phone.glb#Scene0".to_owned(),
        transform,
        0.2,
        ItemId::IronPhone
    )
}

fn spawn_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
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
            enabled: true,
        },
        item: Item {
            id,
            pickup: true,
            desired_transform: transform,
            lerp_active: true,
        }
    });
}
