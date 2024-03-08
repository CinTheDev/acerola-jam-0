use bevy::prelude::*;

use crate::player::items::ItemDropBundle;

#[derive(Bundle)]
pub struct MasterTaskBundle {
    task: MasterTask,
}

#[derive(Bundle)]
pub struct CopperTaskBundle {
    item_drop: ItemDropBundle,
    task: CopperTask,
}

#[derive(Component)]
pub struct MasterTask {
    is_all_done: bool,
}

#[derive(Component)]
pub struct CopperTask {
    is_done: bool,
}
