use bevy::prelude::*;

const LERP_FACTOR: f32 = 0.5;

#[derive(Component)]
pub struct IntroSlide {
    position: i8,
    dist_to_top: f32,
    all_slides_done: bool,
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

pub fn slide_input(
    mut q_slides: Query<&mut IntroSlide>,
    input: Res<Input<KeyCode>>,
) {
    let mut slides_done = true;

    for mut slide in q_slides.iter_mut() {
        if slide.all_slides_done { return }

        if ! input.just_pressed(KeyCode::Space) { return }

        if slide.position < 0 { continue }

        slide.position -= 1;

        slides_done = false;
    }

    if slides_done {
        // TODO: Enable gameplay
        info!("Slides are done");

        for mut slide in q_slides.iter_mut() {
            slide.all_slides_done = true;
        }
    }
}

pub fn slide_slide(
    mut q_slides: Query<(&mut Style, &mut IntroSlide)>,
) {
    for (mut style, mut prop) in q_slides.iter_mut() {
        let desired_pos = prop.position as f32 * 100.0;
        prop.dist_to_top = lerp(prop.dist_to_top, desired_pos, LERP_FACTOR);

        style.top = Val::Percent(prop.dist_to_top);
    }
}

fn spawn_slide(parent: &mut ChildBuilder, id: i8, text: &str) {
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
            position: id,
            dist_to_top: 100.0 * id as f32,
            all_slides_done: false,
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

fn lerp(a: f32, b: f32, factor: f32) -> f32 {
    a * (1.0 - factor) + b * factor
}
