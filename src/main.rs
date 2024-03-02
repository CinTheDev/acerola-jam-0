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
}
