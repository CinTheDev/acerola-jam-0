use bevy::prelude::*;

pub mod ui_cursor;
pub mod ui_timer;
pub mod ui_tasks;
pub mod ui_ending;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
        crate::Respawn,
    )).with_children(|parent| {
        ui_cursor::spawn_ui(parent, asset_server);
        ui_timer::spawn_ui(parent);
        ui_tasks::spawn_ui(parent);
        ui_ending::spawn_ui(parent);
    });
}
