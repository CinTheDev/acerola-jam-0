use bevy::prelude::*;

use crate::timer::{LoseTimer, TimerRunout};

#[derive(Component)]
pub struct TimerText;

pub fn update_timer_ui(
    mut query: Query<&mut Text, With<TimerText>>,
    losetimer: Res<LoseTimer>,
    mut ev_timer_runout: EventReader<TimerRunout>,
) {
    let mut text = query.single_mut();

    let remaining_minutes = losetimer.timer.remaining_secs() as u32 / 60;
    let remaining_seconds = losetimer.timer.remaining_secs() as u32 % 60;
    let remaining_time_string = format!("{:02}:{:02}", remaining_minutes, remaining_seconds);

    let ui_text = text.sections.first_mut().unwrap();

    ui_text.value = remaining_time_string.to_string();

    for _ in ev_timer_runout.read() {
        // Player has lost, timer turns red
        ui_text.style.color = Color::rgb(1.0, 0.0, 0.0).into();
    }
}

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Px(70.0),
            aspect_ratio: Some(2.58),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        background_color: Color::rgb(0.0, 0.0, 0.0).into(),
        ..default()
    }).with_children(|timer_background| {
            timer_background.spawn((TextBundle::from_section(
                "Test",
                TextStyle {
                    font_size: 70.0,
                    ..default()
                },
            ),
            TimerText,
        ));
    });
}
