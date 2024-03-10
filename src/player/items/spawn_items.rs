use bevy::prelude::*;
use super::{collision::SphereCollider, Item, ItemBundle, ItemId};
use crate::RaycastCursor;

pub fn spawn_all_items(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    spawn_dark_matter(commands, asset_server);
    spawn_lead(commands, asset_server);
    spawn_exotic_alloy(commands, asset_server);
    spawn_ironblock(commands, asset_server);
    spawn_ironhammer(commands, asset_server);
    spawn_ironscrewdriver(commands, asset_server);
    spawn_ironphone(commands, asset_server);
    spawn_copperfuel(commands, asset_server);
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
        ItemId::DarkMatter,
        true,
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
        ItemId::Lead,
        true,
    )
}

fn spawn_exotic_alloy(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let transform = Transform::from_xyz(7.5, 0.2, 1.5);

    spawn_item(
        commands,
        asset_server,
        "items/lead.glb#Scene0".to_owned(),
        transform,
        0.2,
        ItemId::ExoticAlloy,
        false,
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
        ItemId::IronBlock,
        true,
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
        ItemId::IronHammer,
        true,
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
        ItemId::IronScrewdriver,
        true,
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
        ItemId::IronPhone,
        true,
    )
}

fn spawn_copperfuel(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let transform = Transform::from_xyz(8.5, 0.95, -6.5);

    spawn_item(
        commands,
        asset_server,
        "items/copper_fuel.glb#Scene0".to_owned(),
        transform,
        0.25,
        ItemId::CopperFuel,
        true,
    );
}

fn spawn_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    asset_path: String,
    transform: Transform,
    radius: f32,
    id: ItemId,
    enabled: bool
) {
    let visibility = match enabled {
        true => Visibility::Inherited,
        false => Visibility::Hidden
    };

    commands.spawn(ItemBundle {
        scene: SceneBundle {
            scene: asset_server.load(asset_path),
            transform,
            visibility,
            ..default()
        },
        collider: SphereCollider {
            radius,
            enabled,
        },
        item: Item {
            id,
            pickup: true,
            desired_transform: transform,
            lerp_active: true,
        },
        r_cursor: RaycastCursor,
        respawn: crate::Respawn,
    });
}
