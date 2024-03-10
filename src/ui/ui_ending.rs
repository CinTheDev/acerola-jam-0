use bevy::prelude::*;
use crate::{player::tasks::q_t_de::FinalButtonActivated, timer::TimerRunout};

pub mod good_ending;
pub mod bad_ending;

#[derive(Component)]
pub struct UIBackground {
    timer: Timer,
}

#[derive(Component)]
pub struct UIGoodEnding {
    lerp_factor: f32,
    lerp_active: bool,
}

#[derive(Component)]
pub struct UIBadEnding {
    lerp_factor: f32,
    lerp_active: bool,
}

pub fn spawn_ui(parent: &mut ChildBuilder) {
    let mut bg_timer = Timer::from_seconds(5.0, TimerMode::Once);
    bg_timer.pause();

    parent.spawn((
        get_background_ui(),
        UIBackground {
            timer: bg_timer,
        },
    ));

    parent.spawn((
        get_ending_ui(),
        UIGoodEnding {
            lerp_factor: 0.3,
            lerp_active: false,
        },
    )).with_children(|root_node| {
        good_ending::spawn_ui(root_node);
    });

    parent.spawn((
        get_ending_ui(),
        UIBadEnding {
            lerp_factor: 0.3,
            lerp_active: false,
        },
    )).with_children(|root_node| {
        bad_ending::spawn_ui(root_node);
    });
}

fn get_background_ui() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
        ..default()
    }
}

fn get_ending_ui() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Percent(10.0)),
            justify_items: JustifyItems::Center,
            bottom: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
}

pub fn fade_background(
    mut query: Query<(&mut BackgroundColor, &mut UIBackground)>,
    time: Res<Time>,
) {
    let (mut color, mut prop) = query.single_mut();

    prop.timer.tick(time.delta());
    let factor = prop.timer.percent();

    color.0.set_a(factor);
}

pub fn check_good_ending(
    mut event: EventReader<FinalButtonActivated>,
    mut q_background: Query<&mut UIBackground>,
) {
    for _ in event.read() {
        info!("Good ending :3");
        let mut prop = q_background.single_mut();
        prop.timer.reset();
        prop.timer.unpause();
    }
}

pub fn check_bad_ending(
    mut event: EventReader<TimerRunout>,
    mut q_background: Query<&mut UIBackground>,
) {
    for _ in event.read() {
        info!("Bad ending :(");
        let mut prop = q_background.single_mut();
        prop.timer.reset();
        prop.timer.unpause();
    }
}
