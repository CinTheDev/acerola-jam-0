use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        SceneBundle {
            scene: asset_server.load("test_env.glb#Scene0"),
            ..default()
        }
    );

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 0.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });
}
