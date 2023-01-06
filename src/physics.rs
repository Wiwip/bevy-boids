use bevy::prelude::*;
use crate::boids::Movement;
use crate::helper::velocity_angle;

pub fn rotation_system(mut query: Query<(&mut Transform, &Movement)>) {
    for (mut tf, mov) in &mut query {
        tf.rotation = Quat::from_rotation_z(velocity_angle(&mov.vel));
    }
}