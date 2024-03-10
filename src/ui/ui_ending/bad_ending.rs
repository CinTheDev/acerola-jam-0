use bevy::prelude::*;

const ENDING_TEXT: &str = "
You were too slow.

The false Vacuum reached Venus and altered the effective gravity on Earth, effectively destroying
all infrastructure and hopes of reverting this entire chain reaction.

In a few minutes the false Vacuum will reach the Sun and destroy every bit of life on Earth from insane
gravitational forces.

The laws of Physics have been changed, the universe will never be the same again.
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
