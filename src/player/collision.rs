use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlaneColliderBundle {
    pub transform: Transform,
    pub collider: PlaneCollider,
}

#[derive(Component)]
pub struct PlaneCollider {
    pub normal: Vec2,
    pub size: f32,
}

#[derive(Component)]
pub struct SphereCollider {
    pub radius: f32,
}

pub fn check_collision_dynamic(
    sphere: &SphereCollider,
    sphere_pos: Vec2,
    plane: &PlaneCollider,
    plane_pos: Vec2,
    velocity: Vec2
) -> (bool, Vec2) {
    // Uses "Collide and Slide" algorithm

    // Project
    let plane_vector_normalized = Vec2::new(-plane.normal.y, plane.normal.x);
    let plane_pos_vector = plane_pos - plane_vector_normalized * plane.size * 0.5;
    let sphere_pos_vector = (sphere_pos + velocity) - plane_pos_vector;

    let project_t = sphere_pos_vector.dot(plane_vector_normalized).min(plane.size).max(0.0);
    let project_vector_plane = plane_vector_normalized * project_t;
    let project_vector_sphere = sphere_pos_vector - project_vector_plane;

    // If the distance between plane and sphere is smaller than radius, there's collision
    if project_vector_sphere.length_squared() > sphere.radius * sphere.radius {
        return (false, velocity);
    }

    // Displace backwards
    let mut reverse_displacement = (sphere.radius - project_vector_sphere.length()) / plane.normal.dot(velocity.normalize() * -1.0);
    if reverse_displacement.is_nan() || reverse_displacement.is_infinite() {
        reverse_displacement = 0.0;
    }
    let reverse_displacement_vector = velocity.normalize_or_zero() * reverse_displacement;
    
    let mut new_velocity = velocity;

    new_velocity -= reverse_displacement_vector;
    
    // Second projection to determine slide
    let slide_project_t = plane_vector_normalized.dot(reverse_displacement_vector);
    let slide_project_vector = plane_vector_normalized * slide_project_t;

    new_velocity += slide_project_vector;

    return (true, new_velocity);
}

// Casts a ray in 3D space and checks for sphere intersections
pub fn raycast(
    ray_pos: Vec3,
    ray_dir: Vec3,
    q_spheres: Query<(&Transform, &SphereCollider)>,
) -> bool {
    for s in q_spheres.iter() {
        let s_trans = s.0;
        let s_coll = s.1;

        let s_pos = s_trans.translation - ray_pos;

        let ray_len = ray_dir.length();
        let ray_norm = ray_dir / ray_len;

        let t = ray_norm.dot(s_pos).max(0.0).min(ray_len);
        let t_pos = ray_norm * t;

        let dist = s_pos - t_pos;

        if dist.length_squared() > s_coll.radius*s_coll.radius { continue; }

        // Sphere has been intersected
        return true;
    }

    return false;
}
