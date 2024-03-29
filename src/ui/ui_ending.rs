use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use crate::{player::Player, timer::{TimerRunout, TimerStop}};

use super::ui_controls::ShowControls;

pub mod good_ending;
pub mod bad_ending;
pub mod buttons;

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
    let mut bg_timer = Timer::from_seconds(1.0, TimerMode::Once);
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
            lerp_factor: 1.0,
            lerp_active: false,
            percent: 100.0,
        },
    )).with_children(|root_node| {
        good_ending::spawn_ui(root_node);
        buttons::spawn_ui(root_node);
    });

    parent.spawn((
        get_ending_ui(),
        UIBadEnding {
            lerp_factor: 1.0,
            lerp_active: false,
            percent: 100.0,
        },
    )).with_children(|root_node| {
        bad_ending::spawn_ui(root_node);
        buttons::spawn_ui(root_node);
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
            display: Display::Flex,
            position_type: PositionType::Absolute,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(Val::Percent(10.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
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
    time: Res<Time>,
) {
    let prop_bg = q_bg.single();
    let (mut style_bad, mut prop_bad) = q_bad.single_mut();
    let (mut style_good, mut prop_good) = q_good.single_mut();

    if ! prop_bg.timer.finished() { return }

    if prop_bad.lerp_active {
        // Multiplying by delta time here is not making this truly framerate independent,
        // but it's a good enough approximation
        prop_bad.percent = lerp(prop_bad.percent, 0.0, prop_bad.lerp_factor * time.delta_seconds());
        style_bad.bottom = Val::Percent(prop_bad.percent);
    }

    if prop_good.lerp_active {
        prop_good.percent = lerp(prop_good.percent, 0.0, prop_good.lerp_factor * time.delta_seconds());
        style_good.bottom = Val::Percent(prop_good.percent);
    }
}

pub fn check_good_ending(
    mut event: EventReader<TimerStop>,
    mut q_background: Query<&mut UIBackground>,
    mut q_text: Query<&mut UIGoodEnding>,
    mut q_player: Query<&mut Player>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut ev_hidectrls: EventWriter<ShowControls>,
) {
    for _ in event.read() {
        let mut prop = q_background.single_mut();
        let mut text = q_text.single_mut();
        let mut player = q_player.single_mut();
        let mut window = q_window.single_mut();

        prop.timer.reset();
        prop.timer.unpause();
        text.lerp_active = true;
        
        player.locked = true;
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        
        ev_hidectrls.send(ShowControls(false));
    }
}

pub fn check_bad_ending(
    mut event: EventReader<TimerRunout>,
    mut q_background: Query<&mut UIBackground>,
    mut q_text: Query<&mut UIBadEnding>,
    mut q_player: Query<&mut Player>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut ev_hidectrls: EventWriter<ShowControls>,
) {
    for _ in event.read() {
        let mut prop = q_background.single_mut();
        let mut text = q_text.single_mut();
        let mut player = q_player.single_mut();
        let mut window = q_window.single_mut();

        prop.timer.reset();
        prop.timer.unpause();
        text.lerp_active = true;

        player.locked = true;
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;

        ev_hidectrls.send(ShowControls(false));
    }
}

fn lerp(a: f32, b: f32, factor: f32) -> f32 {
    a * (1.0 - factor) + b * factor
}
