use bevy::prelude::*;

pub fn instance_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player {
            speed: 3.0
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

pub fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    let mut p = query.single_mut();
    let properties = p.0;
    let transform = p.1.as_mut();

    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        dir += transform.forward();
    }
    if input.pressed(KeyCode::A) {
        dir += transform.left();
    }
    if input.pressed(KeyCode::S) {
        dir += transform.back();
    }
    if input.pressed(KeyCode::D) {
        dir += transform.right();
    }

    dir = dir.normalize_or_zero();

    let vec_move = dir * properties.speed * time.delta_seconds();

    transform.translation += vec_move;
}
