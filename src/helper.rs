use bevy::prelude::*;

pub fn velocity_angle(vel: &Vec2) -> f32 {
    f32::atan2(vel.y, vel.x)
}
