use crate::helper::velocity_angle;
use bevy::math::vec3;
use bevy::prelude::{Quat, Query, Transform, Vec3};
use bevy_rapier2d::prelude::{ExternalForce, RigidBody, Velocity};

pub fn rotation_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut tf, vel) in &mut query {
        tf.rotation = Quat::from_rotation_z(velocity_angle(&vel.linvel));
    }
}
