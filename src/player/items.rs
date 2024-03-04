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
    id: ItemId,
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

pub fn check_item_collision(
    mut q_player: Query<(&Transform, &collision::SphereCollider, &mut super::Player)>,
    mut q_items: Query<(&Transform, &collision::SphereCollider, &mut Item), Without<super::Player>>
) {
    let mut player = q_player.single_mut();
    let player_trans = player.0;
    let player_collider = player.1;
    let player_properties = player.2.as_mut();

    for mut item in q_items.iter_mut() {
        let item_trans = item.0;
        let item_collider = item.1;
        let item_properties = item.2.as_mut();

        if ! item_properties.pickup {
            continue;
        }

        // Check collision
        let sqr_dist = (item_trans.translation - player_trans.translation).length_squared();
        let radii = player_collider.radius + item_collider.radius;

        if sqr_dist > radii * radii {
            continue;
        }

        // Collision
        item_properties.pickup = false;
        item_properties.lerp_active = false;
        player_properties.item_id = item_properties.id;

        return;
    }
}

pub fn check_drop_collision(
    mut q_player: Query<(&Transform, &collision::SphereCollider, &mut super::Player)>,
    mut q_itemdrops: Query<(&Transform, &collision::SphereCollider, &mut ItemDrop), Without<super::Player>>,
    mut q_items: Query<&mut Item, (Without<super::Player>, Without<ItemDrop>)>
) {
    let mut player = q_player.single_mut();
    let player_trans = player.0;
    let player_collider = player.1;
    let player_properties = player.2.as_mut();

    for mut itemdrop in q_itemdrops.iter_mut() {
        let itemdrop_trans = itemdrop.0;
        let itemdrop_collider = itemdrop.1;
        let itemdrop_properties = itemdrop.2.as_mut();

        if itemdrop_properties.accepts_id != player_properties.item_id {
            continue;
        }

        // Check collision
        let sqr_dist = (itemdrop_trans.translation - player_trans.translation).length_squared();
        let radii = itemdrop_collider.radius + player_collider.radius;

        if sqr_dist > radii * radii {
            continue;
        }

        // Drop item
        
        // Search next item and activate
        for mut item in q_items.iter_mut() {
            let item_properties = item.as_mut();

            if item_properties.id == itemdrop_properties.activates_id {
                item_properties.pickup = true;
            }
            if item_properties.id == player_properties.item_id {
                item_properties.desired_transform = itemdrop_trans.clone();
                item_properties.lerp_active = true;
            }
        }

        // TODO: Properly drop of item
        player_properties.item_id = ItemId::None;
        itemdrop_properties.is_dropped = true;

        return;
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