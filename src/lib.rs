extern crate core;

use bevy::prelude::*;

pub mod boids;
pub mod physics;
pub mod predator;
pub mod debug_systems;

pub fn velocity_angle(vel: &Vec3) -> f32 {
    f32::atan2(vel.y, vel.x)
}

