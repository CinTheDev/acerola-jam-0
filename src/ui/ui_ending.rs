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
    percent: f32,
}

#[derive(Component)]
pub struct UIBadEnding {
    lerp_factor: f32,
    lerp_active: bool,
    percent: f32,
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
            lerp_factor: 0.05,
            lerp_active: false,
            percent: 100.0,
        },
    )).with_children(|root_node| {
        good_ending::spawn_ui(root_node);
    });

    parent.spawn((
        get_ending_ui(),
        UIBadEnding {
            lerp_factor: 0.05,
            lerp_active: false,
            percent: 100.0,
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

pub fn swipe_text(
    q_bg: Query<&UIBackground>,
    mut q_bad: Query<(&mut Style, &mut UIBadEnding), Without<UIGoodEnding>>,
    mut q_good: Query<(&mut Style, &mut UIGoodEnding), Without<UIBadEnding>>,
) {
    let prop_bg = q_bg.single();
    let (mut style_bad, mut prop_bad) = q_bad.single_mut();
    let (mut style_good, mut prop_good) = q_good.single_mut();

    if ! prop_bg.timer.finished() { return }

    if prop_bad.lerp_active {
        prop_bad.percent = lerp(prop_bad.percent, 0.0, prop_bad.lerp_factor);
        style_bad.bottom = Val::Percent(prop_bad.percent);
    }

    if prop_good.lerp_active {
        prop_good.percent = lerp(prop_good.percent, 0.0, prop_good.lerp_factor);
        style_good.bottom = Val::Percent(prop_good.percent);
    }
}

pub fn check_good_ending(
    mut event: EventReader<FinalButtonActivated>,
    mut q_background: Query<&mut UIBackground>,
    mut q_text: Query<&mut UIGoodEnding>,
) {
    for _ in event.read() {
        info!("Good ending :3");
        let mut prop = q_background.single_mut();
        let mut text = q_text.single_mut();

        prop.timer.reset();
        prop.timer.unpause();
        text.lerp_active = true;
    }
}

pub fn check_bad_ending(
    mut event: EventReader<TimerRunout>,
    mut q_background: Query<&mut UIBackground>,
    mut q_text: Query<&mut UIBadEnding>,
) {
    for _ in event.read() {
        info!("Bad ending :(");
        let mut prop = q_background.single_mut();
        let mut text = q_text.single_mut();

        prop.timer.reset();
        prop.timer.unpause();
        text.lerp_active = true;
    }
}

fn lerp(a: f32, b: f32, factor: f32) -> f32 {
    a * (1.0 - factor) + b * factor
}
