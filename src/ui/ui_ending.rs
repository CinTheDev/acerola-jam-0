use bevy::prelude::*;

pub mod good_ending;
pub mod bad_ending;

#[derive(Component)]
pub struct EndingUI;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::rgb(0.0, 0.0, 0.0).into(),
        ..default()
    });
}

pub fn hide_ui(
    mut query: Query<&mut Style, With<EndingUI>>,
) {
    for mut style in query.iter_mut() {
        style.display = Display::None;
    }
}
