use bevy::prelude::*;

pub fn spawn_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let first_transform = Transform::from_xyz(-0.609, 0.85, -8.736)
        .with_rotation(Quat::from_rotation_x(28.252_f32.to_radians()));

    // TODO: Spawn 14 Buttons to right and 4 down
}
