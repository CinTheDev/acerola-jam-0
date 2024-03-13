use bevy::prelude::*;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    }).with_children(|bg| {
        spawn_slide(bg);
    });
}

fn spawn_slide(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Percent(10.0)),
            ..default()
        },
        ..default()
    }).with_children(|bg| {
        bg.spawn(TextBundle::from_section(
            "Hi what's up",
            TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            }
        ));
    });
}
