use bevy::prelude::*;

pub mod ui_cursor;
pub mod ui_timer;
pub mod ui_tasks;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        ui_timer::spawn_ui(parent);
        ui_cursor::spawn_ui(parent, asset_server);
    });
}
