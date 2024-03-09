use bevy::prelude::*;

use crate::{
    player::tasks::{
        alloy_machine::{AlloyCreationFinshed, AlloyPlacementFinished},
        computer::SuccessEvent,
        particle_accelerator::ParticleAcceleratorFinished,
        q_t_de::DarkMatterFinished
    },
    timer::TimerStop
};

const TASK_TEXTS: [&'static str; 6] = [
    "Clean Dark Matter from Experiment",
    "Create Exotic Alloy",
    "Place Alloy in Experiment",
    "Prepare Particle Accelerator",
    "Start Computer",
    "Start the Experiment",
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

pub fn check_task_darkmatter(
    mut event: EventReader<DarkMatterFinished>,
) {
    todo!();
}

pub fn check_task_exoticalloy(
    mut event: EventReader<AlloyCreationFinshed>,
) {
    todo!();
}

pub fn check_task_alloyplacement(
    mut event: EventReader<AlloyPlacementFinished>,
) {
    todo!();
}

pub fn check_task_particleaccelerator(
    mut event: EventReader<ParticleAcceleratorFinished>,
) {
    todo!();
}

pub fn check_task_computer(
    mut event: EventReader<SuccessEvent>,
) {
    todo!();
}

pub fn check_task_finalbutton(
    mut event: EventReader<TimerStop>,
) {
    todo!();
}
