use bevy::prelude::*;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            background_color: Color::rgb(1.0, 1.0, 1.0).into(),
            ..default()
        }
    );
}
