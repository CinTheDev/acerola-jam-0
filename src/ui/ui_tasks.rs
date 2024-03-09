use bevy::prelude::*;

const TASK_TEXTS: [&'static str; 4] = [
    "Task 1",
    "Task 2",
    "Task 3",
    "Task 4",
];

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(300.0),
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
            ..default()
        },
    ));
}
