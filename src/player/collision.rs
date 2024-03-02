use bevy::prelude::*;

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
    // Use "Collide and Slide" algorithm
    // TODO

    // Project
    let plane_vector = Vec2::new(-plane.normal.y, plane.normal.x) * plane.size;
    let plane_pos_vector = plane_pos - plane_vector * 0.5;
    let sphere_pos_vector = (sphere_pos + velocity) - plane_pos_vector;

    let plane_vector_length = plane_vector.length();
    let plane_vector_normalized = plane_vector / plane_vector_length;

    let project_t = sphere_pos_vector.dot(plane_vector_normalized).min(plane_vector_length).max(0.0);
    let project_vector_plane = plane_vector_normalized * project_t;
    let project_vector_sphere = sphere_pos_vector - project_vector_plane;

    // If the distance between plane and sphere is smaller than radius, there's collision
    if project_vector_sphere.length_squared() > sphere.radius * sphere.radius {
        return (false, velocity.clone());
    }

    info!("The thing is colliding");

    // Displace backwards
    let reverse_displacement = (sphere.radius - project_vector_sphere.length()) / plane.normal.dot(sphere_pos_vector.normalize() * -1.0);
    let reverse_displacement_vector = velocity.normalize_or_zero() * reverse_displacement * -1.0;

    let mut new_velocity = velocity.clone();
    new_velocity += reverse_displacement_vector;

    // Second projection to determine slide

    return (true, new_velocity);
}
