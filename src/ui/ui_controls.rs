use bevy::prelude::*;

#[derive(Event)]
pub struct HideControls;

#[derive(Event)]
pub struct ShowControls;

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(10.0),
            height: Val::Percent(10.0),
            bottom: Val::ZERO,
            left: Val::ZERO,
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        background_color: Color::RED.into(),
        ..default()
    });
}
