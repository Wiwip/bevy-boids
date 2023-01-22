
use bevy::prelude::*;

pub mod boids;
pub mod physics;

pub fn velocity_angle(vel: &Vec3) -> f32 {
    f32::atan2(vel.y, vel.x)
}

