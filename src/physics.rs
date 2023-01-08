use bevy::prelude::*;
use crate::boids::{BoidsRules, Movement};
use crate::debug_systems::DebugConfig;
use crate::helper::velocity_angle;


pub fn rotation_system(mut query: Query<(&mut Transform, &Movement)>) {
    for (mut tf, mov) in &mut query {
        tf.rotation = Quat::from_rotation_z(velocity_angle(&mov.vel));
    }
}

pub fn move_system(
    mut query: Query<(&mut Transform, &mut Movement)>,
    boid_rules: Res<BoidsRules>,
    debug: Res<DebugConfig>,
    time: Res<Time>
) {
    if debug.freeze_world { return; }
    for (mut tf, mut mov) in &mut query {
        let mut acc = mov.acc;
        // Clamp max acceleration
        if acc.length() > boid_rules.max_force {
            acc = acc / acc.length() * boid_rules.max_force;
        }

        // Apply acceleration changes to velocity.
        mov.vel = mov.vel + acc * time.delta_seconds();

        // Clamp velocity
        let max_vel = 125.0; // TODO move max vel
        if mov.vel.length() > max_vel {
            mov.vel = mov.vel / mov.vel.length() * max_vel;
        }

        tf.translation = tf.translation + mov.vel * time.delta_seconds();
        mov.acc = Vec3::ZERO;
    }

}