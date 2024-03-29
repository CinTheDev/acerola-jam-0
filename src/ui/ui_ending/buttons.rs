use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{timer::ResetTimer, ui::ui_controls::ShowControls, Respawn};

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Event)]
pub struct RestartEvent;

pub fn pressed_button_restart(
    mut event: EventReader<RestartEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_respawn: Query<Entity, With<Respawn>>,
    q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut ev_timerreset: EventWriter<ResetTimer>,
    mut ev_showctrls: EventWriter<ShowControls>,
) {
    for _ in event.read() {
        // Delete all objects marked for Respawn
        for entity_id in q_respawn.iter() {
            commands.entity(entity_id).despawn_recursive();
        }

        // Regenerate items
        crate::player::items::spawn_items::spawn_all_items(&mut commands, &asset_server);

        // Regenerate tasks
        crate::player::tasks::instance_tasks(&mut commands, &asset_server);

        // Regenerate UI
        crate::ui::instance_ui(&mut commands, &asset_server);

        // Reset player position
        crate::player::instance_player(&mut commands);

        // Grab cursor again
        crate::cursor_grab(q_window);

        // Restart timer
        ev_timerreset.send(ResetTimer);

        // Show controls
        ev_showctrls.send(ShowControls(true));

        return;
    }
}

pub fn check_button_restart(
    mut q_interaction: Query<(
        &Interaction,
        &mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>, With<RestartButton>)>,
    mut ev_restart: EventWriter<RestartEvent>,
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
                ev_restart.send(RestartEvent);
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
    mut ev_quit: EventWriter<AppExit>,
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
                ev_quit.send(AppExit);
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
