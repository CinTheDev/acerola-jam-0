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
            (player::items::test_instance_item,
            player::items::test_instance_itemdrop,
            player::items::set_item_desired_transform).chain(),
        ))
        .add_systems(Update, (
            player::move_player,
            player::items::hold_item,
            player::items::update_item_pos,
            player::items::check_item_collision,
            player::items::check_drop_collision,
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("test_env.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    player::instance_player(commands);
}

fn cursor_grab(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = query.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}
