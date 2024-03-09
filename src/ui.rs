use bevy::prelude::*;

pub mod ui_timer;
pub mod ui_cursor;

pub fn setup(mut commands: Commands) {
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
        ui_cursor::spawn_ui(parent);
    });
}
