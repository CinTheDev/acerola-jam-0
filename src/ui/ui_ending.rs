use bevy::prelude::*;
use crate::{player::tasks::q_t_de::FinalButtonActivated, timer::TimerRunout};

pub mod good_ending;
pub mod bad_ending;

#[derive(Component)]
pub struct EndingUI;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::rgb(0.0, 0.0, 0.0).into(),
        ..default()
    }).with_children(|root_node| {
        good_ending::spawn_ui(root_node);
        bad_ending::spawn_ui(root_node);
    });
}

pub fn hide_ui(
    mut query: Query<&mut Style, With<EndingUI>>,
) {
    for mut style in query.iter_mut() {
        style.display = Display::None;
    }
}

pub fn check_good_ending(
    mut event: EventReader<FinalButtonActivated>,
) {
    for _ in event.read() {
        info!("Good ending :3");
    }
}

pub fn check_bad_ending(
    mut event: EventReader<TimerRunout>,
) {
    for _ in event.read() {
        info!("Bad ending :(");
    }
}
