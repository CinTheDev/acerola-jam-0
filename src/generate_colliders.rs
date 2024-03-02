use bevy::prelude::*;
use super::player::collision;

// Let's go for hand input right now, I don't have time to figure out automating it
pub fn generate_colliders(mut commands: Commands) {
    for o in MANUAL_INFORMATION {
        generate_cube(
            &mut commands,
            o.0,
            o.1,
            o.2,
            o.3
        );
    }
}

// Generates 4 planes
fn generate_cube(commands: &mut Commands, position: Vec2, rotation: f32, size: Vec2, inside: bool) {
    // Calculate directions from middle
    let rotation_rad = -rotation.to_radians();
    let mut dir1 = Vec2::new(rotation_rad.cos(), rotation_rad.sin());
    let mut dir2 = Vec2::new(-dir1.y, dir1.x);
    let mut dir3 = Vec2::new(-dir2.y, dir2.x);
    let mut dir4 = Vec2::new(-dir3.y, dir3.x);

    // Positions
    let p1 = position + dir1 * size.x * 0.5;
    let p2 = position + dir2 * size.y * 0.5;
    let p3 = position + dir3 * size.x * 0.5;
    let p4 = position + dir4 * size.y * 0.5;

    // Transforms
    let trans1 = Transform::from_translation(Vec3::new(p1.x, 0.0, p1.y));
    let trans2 = Transform::from_translation(Vec3::new(p2.x, 0.0, p2.y));
    let trans3 = Transform::from_translation(Vec3::new(p3.x, 0.0, p3.y));
    let trans4 = Transform::from_translation(Vec3::new(p4.x, 0.0, p4.y));

    // Normals
    if inside {
        dir1 *= -1.0;
        dir2 *= -1.0;
        dir3 *= -1.0;
        dir4 *= -1.0;
    }

    // Spawn planes
    commands.spawn(collision::PlaneColliderBundle {
        transform: trans1,
        collider: collision::PlaneCollider {
            normal: dir1,
            size: size.y
        }
    });
    commands.spawn(collision::PlaneColliderBundle {
        transform: trans2,
        collider: collision::PlaneCollider {
            normal: dir2,
            size: size.x
        }
    });
    commands.spawn(collision::PlaneColliderBundle {
        transform: trans3,
        collider: collision::PlaneCollider {
            normal: dir3,
            size: size.y
        }
    });
    commands.spawn(collision::PlaneColliderBundle {
        transform: trans4,
        collider: collision::PlaneCollider {
            normal: dir4,
            size: size.x
        }
    });
}

const MANUAL_INFORMATION: [(Vec2, f32, Vec2, bool); 3] = [
    (Vec2::new(0.0, 0.0), 0.0, Vec2::new(10.0, 10.0), true), // Environment
    (Vec2::new(3.0, -3.0), 0.0, Vec2::new(2.0, 2.0), false), // Green cube
    (Vec2::new(-2.41, 2.296), 25.274, Vec2::new(1.0, 1.0), false),
];
