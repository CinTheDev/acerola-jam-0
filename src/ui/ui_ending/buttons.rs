use bevy::prelude::*;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct QuitButton;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn((
        get_button_background(Color::rgb(0.0, 1.0, 0.0)),
        RestartButton,
    ));

    parent.spawn((
        get_button_background(Color::rgb(1.0, 0.0, 0.0)),
        QuitButton,
    ));
}

fn get_button_background(bg: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(300.0),
            height: Val::Px(200.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: bg.into(),
        ..default()
    }
}
