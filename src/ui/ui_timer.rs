use bevy::prelude::*;

use crate::timer::LoseTimer;

#[derive(Component)]
pub struct TimerText;

pub fn update_timer_ui(
    mut query: Query<&mut Text, With<TimerText>>,
    losetimer: Res<LoseTimer>,
) {
    let mut text = query.single_mut();

    let remaining_minutes = losetimer.timer.remaining_secs() as u32 / 60;
    let remaining_seconds = losetimer.timer.remaining_secs() as u32 % 60;
    let remaining_time_string = format!("{:02}:{:02}", remaining_minutes, remaining_seconds);

    text.sections.first_mut().unwrap().value = remaining_time_string.to_string();
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
