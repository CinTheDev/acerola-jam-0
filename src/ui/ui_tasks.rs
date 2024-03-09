use bevy::prelude::*;

use crate::{
    player::tasks::{
        alloy_machine::{AlloyCreationFinshed, AlloyPlacementFinished},
        computer::SuccessEvent,
        particle_accelerator::ParticleAcceleratorFinished,
        q_t_de::{DarkMatterFinished, FinalButtonActivated}
    },
    timer::TimerStop
};

#[derive(Component)]
pub struct TaskText {
    id: usize,
}

#[derive(Clone, Copy)]
enum TaskTextState {
    Inactive,
    Active,
    Finished,
}

const TASK_COUNT: usize = 6;

const TASK_TEXTS: [&'static str; TASK_COUNT] = [
    "Clean the Quantum Tunnel Device",
    "Create Exotic Alloy with Machine",
    "Place Exotic Alloy in Experiment",
    "Prepare the Particle Accelerator",
    "Start and log in to the Computer",
    "Start the Quantum Tunnel Device",
];

const TASK_START_STATES: [TaskTextState; TASK_COUNT] = [
    TaskTextState::Active,
    TaskTextState::Active,
    TaskTextState::Inactive,
    TaskTextState::Active,
    TaskTextState::Active,
    TaskTextState::Inactive,
];

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(200.0),
                margin: UiRect::all(Val::Px(5.0)),
                border: UiRect::all(Val::Px(5.0)),
                display: Display::Grid,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.75).into(),
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
            get_style_from_state(TASK_START_STATES[index])
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
        update_text_style(&mut text, TaskTextState::Finished);
    }
}

pub fn check_task_exoticalloy(
    mut event: EventReader<AlloyCreationFinshed>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(1, query.iter_mut());
        update_text_style(&mut text, TaskTextState::Finished);
        drop(text);

        let mut text_next = get_task(2, query.iter_mut());
        update_text_style(&mut text_next, TaskTextState::Active);
    }
}

pub fn check_task_alloyplacement(
    mut event: EventReader<AlloyPlacementFinished>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(2, query.iter_mut());
        update_text_style(&mut text, TaskTextState::Finished);
    }
}

pub fn check_task_particleaccelerator(
    mut event: EventReader<ParticleAcceleratorFinished>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(3, query.iter_mut());
        update_text_style(&mut text, TaskTextState::Finished);
    }
}

pub fn check_task_computer(
    mut event: EventReader<SuccessEvent>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event.read() {
        let mut text = get_task(4, query.iter_mut());
        update_text_style(&mut text, TaskTextState::Finished);
    }
}

pub fn check_task_finalbutton(
    mut event_timerstop: EventReader<TimerStop>,
    mut event_finalbutton: EventReader<FinalButtonActivated>,
    mut query: Query<(&TaskText, &mut Text)>,
) {
    for _ in event_finalbutton.read() {
        let mut text = get_task(5, query.iter_mut());
        update_text_style(&mut text, TaskTextState::Active);
    }

    for _ in event_timerstop.read() {
        let mut text = get_task(5, query.iter_mut());
        update_text_style(&mut text, TaskTextState::Finished);
    }
}

fn get_style_from_state(state: TaskTextState) -> TextStyle {
    match state {
        TaskTextState::Active => TextStyle {
            font_size: 18.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },

        TaskTextState::Inactive => TextStyle {
            font_size: 18.0,
            color: Color::rgb(0.2, 0.2, 0.2),
            ..default()
        },

        TaskTextState::Finished => TextStyle {
            font_size: 18.0,
            color: Color::rgb(0.0, 0.0, 0.7),
            ..default()
        }
    }
}

fn update_text_style(text: &mut Text, state: TaskTextState) {
    let section = text.sections.first_mut().unwrap();
    section.style = get_style_from_state(state);
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
