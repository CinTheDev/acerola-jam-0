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
