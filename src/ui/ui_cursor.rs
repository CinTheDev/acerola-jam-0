use bevy::prelude::*;

use crate::player::{collision::{raycast, SphereCollider}, items::{Item, ItemDrop, ItemId}, Player};
use crate::RaycastCursor;

#[derive(Component)]
pub struct CursorDefault;

#[derive(Component)]
pub struct CursorSelect;

pub fn check_cursor(
    q_player: Query<(&Transform, &Player)>,
    //q_items: Query<(&Transform, &SphereCollider, &Item)>,
    //q_drops: Query<(&Transform, &SphereCollider, &ItemDrop)>,
    q_points: Query<(&Transform, &SphereCollider, &RaycastCursor)>,
    mut q_cursor_default: Query<&mut Style, (With<CursorDefault>, Without<CursorSelect>)>,
    mut q_cursor_select: Query<&mut Style, (With<CursorSelect>, Without<CursorDefault>)>,
) {
    let (player_trans, player_prop) = q_player.single();

    /*
    let raycast_items = raycast(
        player_trans.translation,
        player_trans.forward() * 5.0,
        q_items.iter()
    );
    let raycast_drops = raycast(
        player_trans.translation,
        player_trans.forward() * 5.0,
        q_drops.iter()
    );
    */
    let raycast = raycast(
        player_trans.translation,
        player_trans.forward() * 5.0,
        q_points.iter()
    );

    //let player_holding_item = player_prop.item_id != ItemId::None;
    let mut cursor_default = q_cursor_default.single_mut();
    let mut cursor_select = q_cursor_select.single_mut();

    update_cursor(raycast.is_some(), &mut cursor_default, &mut cursor_select);

    /*
    if player_holding_item {
        update_cursor(raycast_drops.is_some(), cursor_default.as_mut(), cursor_select.as_mut());
    }
    else {
        update_cursor(raycast_items.is_some(), cursor_default.as_mut(), cursor_select.as_mut());
    }
    */
}

fn update_cursor(
    select: bool,
    cursor_default: &mut Style,
    cursor_select: &mut Style,
) {
    if select {
        cursor_default.display = Display::None;
        cursor_select.display = Display::Grid;
    }
    else {
        cursor_select.display = Display::None;
        cursor_default.display = Display::Grid;
    }
}

pub fn spawn_ui(parent: &mut ChildBuilder, asset_server: Res<AssetServer>) {
    // Default cursor
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|root| {
        root.spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load("img/cursor_default.png")),
                ..default()
            },
            CursorDefault,
        ));
    });

    // Select cursor
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|root| {
        root.spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load("img/cursor_select.png")),
                ..default()
            },
            CursorSelect,
        ));
    });
}
