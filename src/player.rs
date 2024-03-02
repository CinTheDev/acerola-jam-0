use bevy::prelude::*;

pub fn instance_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player {
            speed: 0.5
        },
        camera: Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        }
    });
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    camera: Camera3dBundle,
}

#[derive(Component)]
pub struct Player {
    speed: f32
}

pub fn move_player(time: Res<Time>, mut query: Query<(&Player, &mut Transform)>) {
    let mut p = query.single_mut();
    let properties = p.0;
    let transform = p.1.as_mut();

    let dir = Vec3::new(1.0, 0.0, 0.0);
    transform.translation += dir * properties.speed * time.delta_seconds();
}
