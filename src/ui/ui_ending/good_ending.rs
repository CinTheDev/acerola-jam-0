use bevy::prelude::*;

const ENDING_TEXT: &str = "
This is some test text
Yeah
";

pub fn spawn_ui(parent: &mut ChildBuilder) {
    parent.spawn(TextBundle::from_section(
        ENDING_TEXT,
        TextStyle {
            font_size: 24.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        }
    ));
}
