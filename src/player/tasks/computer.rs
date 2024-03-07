use bevy::prelude::*;

use crate::player::collision::SphereCollider;

#[derive(Bundle)]
struct ComputerTaskBundle {
    collider: SphereCollider,
    task: ComputerTask,
}

#[derive(Component)]
struct ComputerTask {

}
