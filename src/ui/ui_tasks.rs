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
                margin: UiRect::all(Val::Px(5.0)),
                border: UiRect::all(Val::Px(5.0)),
                display: Display::Grid,
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
            ..default()
        },
    )).with_children(|task_root| {
        for text in TASK_TEXTS {
            spawn_task_text(task_root, text);
        }
    });
}

fn spawn_task_text(parent: &mut ChildBuilder, text: &str) {
    parent.spawn(
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 18.0,
                ..default()
            }
        )
    );
}
