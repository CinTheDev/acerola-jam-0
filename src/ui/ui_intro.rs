use bevy::prelude::*;

use crate::{player::Player, sound::{PlaySoundEvent, SoundID}, timer::LoseTimer};

const LERP_FACTOR: f32 = 0.1;

#[derive(Component)]
pub struct IntroSlide {
    position: i8,
    dist_to_top: f32,
    all_slides_done: bool,
}

#[derive(Event)]
pub struct SlidesFinishedEvent;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    }).with_children(|bg| {
        spawn_slide(bg, 0, "Your science experiment has failed. (Or succeeded?)");
        spawn_slide(bg, 1, "There's a bubble-shaped false Vacuum expanding at light speed now.");
        spawn_slide(bg, 2, "Inside the false Vacuum, Gravity doesn't decrease with distance, meaning far away planets will pull objects with similar strength as earth's pull.");
        spawn_slide(bg, 3, "If the false Vacuum reaches Venus, its gravitational pull will certainly destroy earth.");
        spawn_slide(bg, 4, "To prevent this, you must redo the science experiment.");
        spawn_slide(bg, 5, "There are tasks listed in the corner, do all of them.");
        spawn_slide(bg, 6, "There are 3 machines:");
        spawn_slide(bg, 7, "The Quantum Tunnel Device is in the middle of the room on a table.");
        spawn_slide(bg, 8, "The Alloy Machine is the tall machine with a single display.");
        spawn_slide(bg, 9, "The Particle accelerator is in the adjacent room.");
        spawn_slide(bg, 10, "Good luck.");
    });
}

pub fn slide_input(
    mut q_slides: Query<&mut IntroSlide>,
    input: Res<Input<KeyCode>>,
    mut ev_finished: EventWriter<SlidesFinishedEvent>,
) {
    let mut slides_done = true;

    for mut slide in q_slides.iter_mut() {
        if slide.all_slides_done { return }

        if ! input.just_pressed(KeyCode::Space) { return }

        slide.position -= 1;
        
        if slide.position < 0 { continue }

        slides_done = false;
    }

    if slides_done {
        ev_finished.send(SlidesFinishedEvent);
        info!("Slides are done");

        for mut slide in q_slides.iter_mut() {
            slide.position = -2; // Move them far away from the screen
            slide.all_slides_done = true;
        }
    }
}

pub fn slide_slide(
    mut q_slides: Query<(&mut Style, &mut IntroSlide)>,
) {
    for (mut style, mut prop) in q_slides.iter_mut() {
        let desired_pos = prop.position as f32 * 100.0;
        prop.dist_to_top = lerp(prop.dist_to_top, desired_pos, LERP_FACTOR);

        style.top = Val::Percent(prop.dist_to_top);
    }
}

pub fn finish_slides(
    mut ev_finished: EventReader<SlidesFinishedEvent>,
    mut ev_startmusic: EventWriter<PlaySoundEvent>,
    mut q_player: Query<&mut Player>,
    mut lose_timer: ResMut<LoseTimer>,
) {
    for _ in ev_finished.read() {
        // Unlock Player
        let mut player = q_player.single_mut();
        player.locked = false;

        // Start music
        ev_startmusic.send(PlaySoundEvent(SoundID::Music));

        // Start timer
        lose_timer.timer.unpause();
    }
}

fn spawn_slide(parent: &mut ChildBuilder, id: i8, text: &str) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Percent(10.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        },
        IntroSlide {
            position: id,
            dist_to_top: 100.0 * id as f32,
            all_slides_done: false,
        }
    )).with_children(|bg| {
        bg.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            }
        ));
    });
}

fn lerp(a: f32, b: f32, factor: f32) -> f32 {
    a * (1.0 - factor) + b * factor
}
