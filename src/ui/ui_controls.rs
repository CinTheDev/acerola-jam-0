use bevy::prelude::*;

use crate::player::{items::ItemId, Player};

use super::ui_intro::IntroSlide;

#[derive(Event)]
pub struct ShowControls(pub bool);

#[derive(Component)]
pub struct ControlsUI;

#[derive(Component)]
pub struct ControlsText;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(10.0),
                height: Val::Percent(10.0),
                bottom: Val::ZERO,
                left: Val::ZERO,
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            ..default()
        },
        ControlsUI,
    )).with_children(|bg| {
        bg.spawn((
            TextBundle::from_section(
                "Default text",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                }
            ),
            ControlsText,
        ));
    });
}

pub fn show_controls(
    mut ev_show: EventReader<ShowControls>,
    mut q_ctrls: Query<&mut Style, With<ControlsUI>>,
) {
    for show in ev_show.read() {
        let mut style = q_ctrls.single_mut();

        if show.0 {
            style.display = Display::DEFAULT;
        }
        else {
            style.display = Display::None;
        }
    }
}

pub fn change_controls_text(
    mut q_ctrls: Query<&mut Text, With<ControlsText>>,
    q_player: Query<&Player>,
    q_slides: Query<&IntroSlide>,
) {
    let mut text = q_ctrls.single_mut();
    let player = q_player.single();
    let slides = q_slides.iter();

    let mut ctrl_text = get_regular_text();

    if player.item_id != ItemId::None {
        ctrl_text = get_item_text();
    }

    if ! slides.last().unwrap().all_slides_done {
        ctrl_text = get_slides_text();
    }

    // Set text according to state
    text.sections.first_mut().unwrap().value = ctrl_text;
}

fn get_regular_text() -> String {
    "F - Interact / Grab".to_owned()
}

fn get_item_text() -> String {
    "F - Drop\nX - Return".to_owned()
}

fn get_slides_text() -> String {
    "Space - Continue".to_owned()
}
