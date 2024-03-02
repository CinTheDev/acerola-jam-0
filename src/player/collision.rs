use bevy::prelude::*;

#[derive(Component)]
pub struct PlaneCollider {
    pub normal: Vec3,
    pub size: Vec2,
}

#[derive(Component)]
pub struct SphereCollider {
    pub position: Vec3,
    pub radius: f32,
}

pub fn check_collision_dynamic(
    sphere: &SphereCollider,
    sphere_trans: &Transform,
    plane: &PlaneCollider,
    plane_trans: &Transform,
    velocity: &Vec3
) -> (bool, Vec3) {
    // Use "Collide and Slide" algorithm
    // TODO

    return (false, velocity.clone());
}
