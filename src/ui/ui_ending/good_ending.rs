use bevy::prelude::*;

const ENDING_TEXT: &str = 
"You did it!

By redoing the experiment that started all of this in the first place, \
you effectively reverted all modifications of physics, which definetly \
would've destroyed all life on earth otherwise.

However, the false Vauum Decay is still expanding (even if it's 'hollow' now), \
Earth won't be affected, but some unlucky stars might get obliterated if they \
enter the false vacuum at the same time as other big celestial objects.";

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
