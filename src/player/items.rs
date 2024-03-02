use bevy::prelude::*;
use super::collision;

pub enum ITEM_ID {
    NONE,
    SOMETHING,
}

#[derive(Bundle)]
struct ItemBundle {
    scene: SceneBundle,
    collider: collision::SphereCollider,
    item: Item,
}

#[derive(Component)]
struct Item {
    id: ITEM_ID,
}
