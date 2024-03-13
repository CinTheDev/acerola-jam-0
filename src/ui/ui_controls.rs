use bevy::prelude::*;

#[derive(Event)]
pub struct ShowControls(pub bool);

#[derive(Component)]
pub struct ControlsText;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(10.0),
                height: Val::Percent(10.0),
                bottom: Val::ZERO,
                left: Val::ZERO,
                position_type: PositionType::Absolute,
                display: Display::None,
                margin: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        },
        ControlsText,
    ));
}

pub fn show_controls(
    mut ev_show: EventReader<ShowControls>,
    mut q_ctrls: Query<&mut Style, With<ControlsText>>,
) {
    for show in ev_show.read() {
        let mut style = q_ctrls.single_mut();

        if show.0 {
            style.display = Display::DEFAULT;
        }
        else {
            style.display = Display::None;
        }
    }
}
