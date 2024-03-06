use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

mod player;
mod generate_colliders;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup,
            cursor_grab,
            generate_colliders::generate_colliders,
            player::items::test_instance_item,
            player::tasks::instance_tasks,
        ))
        .add_systems(Update, (
            player::move_player,
            player::raycast_items,
            player::items::hold_item,
            player::items::update_item_pos,
            player::items::pickup_item,
            player::items::drop_item,
            player::items::cancel_itemdrop,
            //player::items::check_item_collision,
            //player::items::check_drop_collision,
            player::tasks::test_task::check_if_dropped,
            player::tasks::test_task::do_task,
        ))
        .add_event::<player::items::PickupEvent>()
        .add_event::<player::items::DropCancelEvent>()
        .add_event::<player::items::DropEvent>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("lab.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    player::items::spawn_items::spawn_all_items(&mut commands, asset_server);
    player::instance_player(&mut commands);
}

fn cursor_grab(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = query.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}
