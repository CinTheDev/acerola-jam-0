use bevy::prelude::*;
use super::collision;
use once_cell::sync::Lazy;

const ITEM_LERP_FACTOR: f32 = 0.5;

static ITEM_HOLD_TRANSFORM: Lazy<Transform> = Lazy::new(|| {
    Transform::from_xyz(0.15, -0.15, -0.3)
        .with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0))
});

#[derive(PartialEq, Clone, Copy)]
pub enum ItemId {
    None,
    Something,
    SomethingElse,
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub scene: SceneBundle,
    pub collider: collision::SphereCollider,
    pub item: Item,
}

#[derive(Component)]
pub struct Item {
    pub id: ItemId,
    pickup: bool,
    desired_transform: Transform,
    lerp_active: bool,
}

#[derive(Bundle)]
pub struct ItemDropBundle {
    pub collider: collision::SphereCollider,
    pub item_drop: ItemDrop,
}

#[derive(Component)]
pub struct ItemDrop {
    pub accepts_id: ItemId,
    pub activates_id: ItemId,
    pub is_dropped: bool,
}

#[derive(Event)]
pub struct PickupEvent(pub ItemId);

#[derive(Event)]
pub struct DropCancelEvent();

#[derive(Event)]
pub struct DropEvent(pub ItemId);

pub fn update_item_pos(mut query: Query<(&mut Transform, &Item)>) {
    for mut item in query.iter_mut() {
        let item_trans = item.0.as_mut();
        let item_properties = item.1;

        if ! item_properties.lerp_active {
            continue;
        }

        // Lerp to desired transform
        *item_trans = lerp_item_towards(&item_trans, &item_properties.desired_transform);
    }
}

pub fn hold_item(
    mut q_item: Query<(&Item, &mut Transform), Without<super::Player>>,
    q_player: Query<(&super::Player, &Transform), Without<Item>>
) {
    let player = q_player.single();
    let player_properties = player.0;
    let player_transform = player.1;

    for mut item in q_item.iter_mut() {
        let item_properties = item.0;
        let item_trans = item.1.as_mut();

        // Check if item is held
        if item_properties.id != player_properties.item_id {
            continue;
        }

        // If yes, do some transformation magix
        let desired_transform = *player_transform * *ITEM_HOLD_TRANSFORM;
        *item_trans = lerp_item_towards(item_trans, &desired_transform);
    }
}

pub fn test_instance_item(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ItemBundle {
        scene: SceneBundle {
            scene: asset_server.load("test_item.glb#Scene0"),
            transform: Transform::from_xyz(3.0, 1.0, 3.0),
            ..default()
        },
        collider: collision::SphereCollider {
            radius: 0.1
        },
        item: Item {
            id: ItemId::Something,
            pickup: true,
            desired_transform: Transform::IDENTITY,
            lerp_active: false,
        }
    });
}

pub fn pickup_item(
    mut ev_pickup: EventReader<PickupEvent>,
    mut q_player: Query<&mut super::Player>,
    mut q_items: Query<&mut Item>,
) { 
    for ev in ev_pickup.read() {
        let mut player = q_player.single_mut();
        let item_id = ev.0;
        info!("Pickup Event");

        if player.item_id != ItemId::None { return; }

        // Search for item
        for mut i in q_items.iter_mut() {
            if i.id != item_id { continue }

            i.pickup = false;
            i.lerp_active = false;
            player.item_id = i.id;

            return;
        }
    }
}

pub fn cancel_itemdrop(
    mut ev_cancel: EventReader<DropCancelEvent>,
    mut q_player: Query<&mut super::Player>,
    mut q_items: Query<&mut Item>,
) {
    for _ev in ev_cancel.read() {
        let mut player = q_player.single_mut();
        let item_id = player.item_id;
        info!("Cancel event");

        if player.item_id == ItemId::None { return }

        for mut i in q_items.iter_mut() {
            if i.id != item_id { continue }

            i.pickup = true;
            i.lerp_active = true;
            player.item_id = ItemId::None;

            return;
        }
    }
}

pub fn drop_item(
    mut ev_itemdrop: EventReader<DropEvent>,
    mut q_player: Query<&mut super::Player>,
    mut q_items: Query<&mut Item>,
    mut q_itemdrops: Query<&mut ItemDrop>,
) {
    for ev in ev_itemdrop.read() {
        let item_id = ev.0;
        let mut player = q_player.single_mut();
        info!("Drop Event");

        if player.item_id == ItemId::None { return }

        for mut i in q_items.iter_mut() {
            if i.id != item_id { continue }

            i.lerp_active = true;
            break;
        }

        for mut d in q_itemdrops.iter_mut() {
            if d.accepts_id != item_id { continue }

            d.is_dropped = true;
            break;
        }

        // TODO: Properly drop of item
        // Drop item
        //item_properties.pickup = true; Activate next item pickup
        //item.desired_transform = [Itemdrop transform];
        player.item_id = ItemId::None;
    }
}

fn lerp_item_towards(item_transform: &Transform, desired_transform: &Transform) -> Transform {
    let new_translation = item_transform.translation.lerp(
        desired_transform.translation, ITEM_LERP_FACTOR
    );
    let new_rotation = item_transform.rotation.slerp(
        desired_transform.rotation, ITEM_LERP_FACTOR
    );
    let new_scale = item_transform.scale.lerp(
        desired_transform.scale, ITEM_LERP_FACTOR
    );

    return Transform {
        translation: new_translation,
        rotation: new_rotation,
        scale: new_scale,
    };
}
