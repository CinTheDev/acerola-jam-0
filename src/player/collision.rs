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
    sphere: SphereCollider,
    sphere_trans: Vec3,
    cube: BoxCollider,
    cube_trans: Vec3,
    velocity: Vec3
) -> Vec3 {
    // Use "Collide and Slide" algorithm
    // TODO

    return velocity;
}
