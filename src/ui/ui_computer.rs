use bevy::prelude::*;

use crate::player::tasks::computer::ComputerTask;

#[derive(Component)]
pub struct ComputerScreenText;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            background_color: Color::rgb(1.0, 1.0, 1.0).into(),
            ..default()
        }
    ).with_children(|bg| {
        bg.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 30.0,
                    color: Color::BLACK,
                    ..default()
                }
            ),
            ComputerScreenText,
        ));
    });
}

pub fn computer_screen_text(
    q_computer_task: Query<&ComputerTask>,
    mut q_screen_text: Query<&mut Text, With<ComputerScreenText>>
) {
    let computer_task = q_computer_task.single();
    let mut screen_text = q_screen_text.single_mut();

    let input_text_length = computer_task.input.len();

    let mut display_text = "".to_owned();

    for _ in 0..input_text_length {
        display_text += "*";
    }

    screen_text.sections.first_mut().unwrap().value = display_text;
}
