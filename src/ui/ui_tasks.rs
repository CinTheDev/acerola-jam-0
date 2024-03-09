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

#[derive(Component)]
pub struct TaskText {
    id: usize,
}

const TASK_COUNT: usize = 6;

const TASK_TEXTS: [&'static str; TASK_COUNT] = [
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
        for i in 0..TASK_COUNT {
            spawn_task_text(task_root, i);
        }
    });
}

fn spawn_task_text(parent: &mut ChildBuilder, index: usize) {
    parent.spawn((
        TextBundle::from_section(
            TASK_TEXTS[index],
            TextStyle {
                font_size: 18.0,
                ..default()
            }
        ),
        TaskText {
            id: index,
        }
    ));
}

pub fn check_task_darkmatter(
    mut event: EventReader<DarkMatterFinished>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(0, query.iter_mut());
        cross_out_text(&mut text);
    }
}

pub fn check_task_exoticalloy(
    mut event: EventReader<AlloyCreationFinshed>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(1, query.iter_mut());
        cross_out_text(&mut text);
    }
}

pub fn check_task_alloyplacement(
    mut event: EventReader<AlloyPlacementFinished>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(2, query.iter_mut());
        cross_out_text(&mut text);
    }
}

pub fn check_task_particleaccelerator(
    mut event: EventReader<ParticleAcceleratorFinished>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(3, query.iter_mut());
        cross_out_text(&mut text);
    }
}

pub fn check_task_computer(
    mut event: EventReader<SuccessEvent>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(4, query.iter_mut());
        cross_out_text(&mut text);
    }
}

pub fn check_task_finalbutton(
    mut event: EventReader<TimerStop>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(5, query.iter_mut());
        cross_out_text(&mut text);
    }
}

fn cross_out_text(text: &mut Text) {
    let section = text.sections.first_mut().unwrap();
    section.style.color = Color::rgb(0.0, 0.0, 1.0);
}

fn get_task<'a, I>(
    id: usize,
    query: I
) -> Mut<'a, Text>
where
    I: Iterator<Item = (&'a TaskText, Mut<'a, Text>)>
{
    for (properties, text) in query {
        if properties.id == id {
            return text;
        }
    }

    panic!("Task Text not found");
}
