use bevy::prelude::*;
use super::collision;
use once_cell::sync::Lazy;

static ITEM_HOLD_TRANSFORM: Lazy<Transform> = Lazy::new(|| {
    Transform::from_xyz(0.0, 0.0, 0.0)
        .with_rotation(Quat::from_euler(EulerRot::YXZ, 1.0, 1.0, 0.0))
});

#[derive(PartialEq)]
pub enum ITEM_ID {
    NONE,
    SOMETHING,
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub scene: SceneBundle,
    pub collider: collision::SphereCollider,
    pub item: Item,
}

#[derive(Component)]
pub struct Item {
    id: ITEM_ID,
    pickup: bool,
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
        let trans = item.1.as_mut();

        // Check if item is held
        if item_properties.id != player_properties.item_id {
            continue;
        }

        // If yes, do some transformation magix
        *trans = *player_transform * *ITEM_HOLD_TRANSFORM;
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
            id: ITEM_ID::SOMETHING,
            pickup: true
        }
    });
}
