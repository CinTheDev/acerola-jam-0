use bevy::prelude::*;

#[derive(Component)]
pub struct BoxCollider {
    pub transform: Transform,
}

#[derive(Component)]
pub struct SphereCollider {
    pub position: Vec3,
    pub radius: f32,
}

pub fn check_collision_dynamic(
    sphere: &SphereCollider,
    sphere_trans: &Transform,
    cube: &BoxCollider,
    cube_trans: &Transform,
    velocity: &Vec3
) -> (bool, Vec3) {
    // Use "Collide and Slide" algorithm
    // TODO

    return (false, velocity.clone());
}
