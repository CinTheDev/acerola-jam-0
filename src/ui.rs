use bevy::prelude::*;

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
        // Timer
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
            ..default()
        }).with_children(|timer_background| {
            timer_background.spawn(TextBundle::from_section(
                "Test",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            ));
        });
    });
}
