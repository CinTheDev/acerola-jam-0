// Structure and general implementation of tasks
use bevy::prelude::*;

use super::{collision, items::{ItemDrop, ItemDropBundle, ItemId}};

pub mod test_task;

pub fn instance_tasks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(test_task::TestTaskBundle {
        scene: SceneBundle {
            scene: asset_server.load("test_item.glb#Scene0"),
            transform: Transform::from_xyz(-3.0, 1.0, -3.0),
            ..default()
        },
        item_drop: ItemDropBundle {
            collider: collision::SphereCollider {
                radius: 0.1
            },

            item_drop: ItemDrop {
                accepts_id: ItemId::Something,
                activates_id: ItemId::SomethingElse,
                is_dropped: false,
            }
        },
        test_task: test_task::TestTask {
            is_active: false,
            needs_check: true,
        }
    });
}
