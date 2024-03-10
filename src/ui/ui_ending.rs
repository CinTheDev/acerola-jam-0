use bevy::prelude::*;

#[derive(Component)]
pub struct EndingUI;

pub fn spawn_ui(parent: &mut ChildBuilder) {

}

pub fn hide_ui(
    mut query: Query<&mut Style, With<EndingUI>>,
) {
    for mut style in query.iter_mut() {
        style.display = Display::None;
    }
}
