use bevy::prelude::*;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Event)]
pub struct RestartEvent;

#[derive(Event)]
pub struct QuitEvent;

pub fn pressed_button_restart(
    mut event: EventReader<RestartEvent>,
) {
    for _ in event.read() {

    }
}

pub fn pressed_button_quit(
    mut event: EventReader<QuitEvent>,
) {
    for _ in event.read() {
        
    }
}

pub fn check_button_restart(
    mut q_interaction: Query<(
        &Interaction,
        &mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>, With<RestartButton>)>,
) {
    for (interaction, mut color) in q_interaction.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = Color::rgb(0.0, 0.2, 0.0).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.0, 0.4, 0.0).into();
            }
            Interaction::Pressed => {
                *color = Color::rgb(0.0, 0.1, 0.0).into();
                // TODO: Implement restart
            }
        }
    }
}

pub fn check_button_quit(
    mut q_interaction: Query<(
        &Interaction,
        &mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>, With<QuitButton>)>,
) {
    for (interaction, mut color) in q_interaction.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = Color::rgb(0.2, 0.0, 0.0).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.4, 0.0, 0.0).into();
            }
            Interaction::Pressed => {
                *color = Color::rgb(0.1, 0.0, 0.0).into();
                // TODO: Implement quit
            }
        }
    }
}

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn((
        get_button_background(Color::rgb(0.0, 0.2, 0.0)),
        RestartButton,
    )).with_children(|bg| {
        bg.spawn(get_button_text("Restart"));
    });

    parent.spawn((
        get_button_background(Color::rgb(0.2, 0.0, 0.0)),
        QuitButton,
    )).with_children(|bg| {
        bg.spawn(get_button_text("Quit"));
    });
}

fn get_button_background(bg: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            height: Val::Px(100.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: bg.into(),
        ..default()
    }
}

fn get_button_text(text: &str) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: 30.0,
            color: Color::rgb(0.8, 0.8, 0.8),
            ..default()
        }
    )
}
