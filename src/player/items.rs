use bevy::prelude::*;
use super::collision;

#[derive(Bundle)]
struct ItemBundle {
    scene: SceneBundle,
    collider: collision::SphereCollider,
    item: Item,
}

#[derive(Component)]
struct Item {
    is_held: bool,
}
