use bevy::prelude::*;

#[derive(Component)]
pub struct TimerText;

pub fn update_timer_ui(
    mut query: Query<&mut Text, With<TimerText>>,
) {
    let mut text = query.single_mut();

    text.sections.first_mut().unwrap().value = "Hi".to_string();
}

pub fn spawn_ui(parent: &mut ChildBuilder) {
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
            timer_background.spawn((TextBundle::from_section(
                "Test",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            ),
            TimerText,
        ));
    });
}
