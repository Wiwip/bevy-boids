use crate::behaviours::DesiredVelocity;
use crate::flock::{BoidsRules, SteeringPressure};
use crate::physics::Velocity;
use bevy::prelude::*;

pub fn desired_velocity_system(
    query: Query<(&Velocity, &DesiredVelocity, &SteeringPressure)>,
    rules: Res<BoidsRules>,
) {
    for (vel, des, steer) in &query {
        let delta_vel = rules.desired_speed - vel.vec.length();
        let unit_vel = vel.vec / vel.vec.length();

        if !unit_vel.is_nan() {
            let force = unit_vel * delta_vel;
            let mut vec = steer.lock.write().unwrap();
            *vec += force * des.factor;
        }
    }
}
