use bevy::prelude::*;

#[derive(Event)]
pub struct ShowControls(pub bool);

#[derive(Component)]
pub struct ControlsUI;

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
            ..default()
        },
        ControlsUI,
    )).with_children(|bg| {
        bg.spawn((
            TextBundle::from_section(
                "Default text",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                }
            ),
            ControlsText,
        ));
    });
}

pub fn show_controls(
    mut ev_show: EventReader<ShowControls>,
    mut q_ctrls: Query<&mut Style, With<ControlsUI>>,
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
