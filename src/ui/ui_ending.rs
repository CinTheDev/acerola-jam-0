use bevy::prelude::*;
use crate::{player::tasks::q_t_de::FinalButtonActivated, timer::TimerRunout};

pub mod good_ending;
pub mod bad_ending;

#[derive(Component)]
pub struct UIBackground {

}

#[derive(Component)]
pub struct UIGoodEnding {

}

#[derive(Component)]
pub struct UIBadEnding {

}

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn((
        get_background_ui(),
        UIBackground {

        },
    ));

    parent.spawn((
        get_ending_ui(),
        UIGoodEnding {

        },
    )).with_children(|root_node| {
        good_ending::spawn_ui(root_node);
    });

    parent.spawn((
        get_ending_ui(),
        UIBadEnding {
            
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
            display: Display::Grid,
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
            display: Display::None,
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Percent(10.0)),
            justify_items: JustifyItems::Center,
            bottom: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
}

pub fn check_good_ending(
    mut event: EventReader<FinalButtonActivated>,
    mut query: Query<&mut Style, With<UIGoodEnding>>,
) {
    for _ in event.read() {
        info!("Good ending :3");
        let mut style = query.single_mut();
        style.display = Display::Grid;
    }
}

pub fn check_bad_ending(
    mut event: EventReader<TimerRunout>,
    mut query: Query<&mut Style, With<UIBadEnding>>,
) {
    for _ in event.read() {
        info!("Bad ending :(");
        let mut style = query.single_mut();
        style.display = Display::Grid;
    }
}
