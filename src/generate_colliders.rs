use bevy::prelude::*;
use super::player::collision;

// Let's go for hand input right now, I don't have time to figure out automating it
pub fn generate_colliders(mut commands: Commands) {
    for o in MANUAL_INFORMATION_BOXES {
        generate_cube(
            &mut commands,
            o.0,
            o.1,
            o.2,
            o.3
        );
    }

    for p in MANUAL_INFORMATION_PLANES {
        generate_plane(
            &mut commands,
            p.0,
            p.1
        );
    }
}

fn generate_plane(commands: &mut Commands, p1: Vec2, p2: Vec2) {
    let dir = p2 - p1;

    let position = p1 + dir * 0.5;
    let normal = Vec2::new(-dir.y, dir.x).normalize();

    commands.spawn(collision::PlaneColliderBundle {
        transform: Transform::from_translation(Vec3::new(position.x, 0.0, position.y)),
        collider: collision::PlaneCollider {
            normal: normal,
            size: dir.length(),
        }
    });
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

const MANUAL_INFORMATION_BOXES: [(Vec2, f32, Vec2, bool); 6] = [
    (Vec2::new(0.0, 0.0), 0.0, Vec2::new(16.0, 20.0), true), // Walls
    (Vec2::new(0.0, 3.0), 0.0, Vec2::new(6.0, 6.0), false), // Table in middle
    (Vec2::new(3.0, -9.0), 0.0, Vec2::new(8.0, 2.0), false), // Controls
    (Vec2::new(-5.0, 9.25), 0.0, Vec2::new(1.0, 1.0), false), // Trashcan
    (Vec2::new(3.5, -4.5), 0.0, Vec2::new(13.0, 1.0), false), // Seperating Wall right
    (Vec2::new(-8.25, -4.5), 0.0, Vec2::new(3.5, 1.0), false), // Seperating Wall left
];

const MANUAL_INFORMATION_PLANES: [(Vec2, Vec2); 3] = [
    (Vec2::new(7.5, -0.5), Vec2::new(7.5, 3.5)), // Alloy machine
    (Vec2::new(10.0, 8.0), Vec2::new(5.0, 8.0)), // Table corner upper
    (Vec2::new(5.0, 8.0), Vec2::new(5.0, 10.0)), // Table corner left
];
