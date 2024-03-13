use bevy::prelude::*;

const LERP_FACTOR: f32 = 0.5;

#[derive(Component)]
pub struct IntroSlide {
    id: u8,
    dist_to_top: f32,
}

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    }).with_children(|bg| {
        spawn_slide(bg, 0, "Hi what's up");
    });
}

fn spawn_slide(parent: &mut ChildBuilder, id: u8, text: &str) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Percent(10.0)),
                ..default()
            },
            ..default()
        },
        IntroSlide {
            id,
            dist_to_top: 100.0 * id as f32,
        }
    )).with_children(|bg| {
        bg.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            }
        ));
    });
}
