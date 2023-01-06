use bevy::math::vec3;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_debug_lines::DebugLines;
use rand_distr::num_traits::{Pow, pow};
use crate::debug_systems::DebugConfig;

#[derive(Resource, Inspectable, Default)]
pub struct BoidsRules {
    pub perception_range: f32,
    pub desired_separation: f32,
    pub coherence_factor: f32,
    pub alignment_factor: f32,
    pub separation_factor: f32,
    pub desired_speed: f32,
    pub stay_inside: f32,
    pub max_force: f32,
    pub(crate) velocity_match_factor: f32,
}

#[derive(Resource)]
pub struct GameRules {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub particle_count: u32,
}

#[derive(Component, Inspectable, Default)]
pub struct Movement {
    pub vel: Vec3,
}

#[derive(Component)]
pub struct Boid;

#[derive(Component, Default)]
pub struct BoidsCoherence {
    pub force: Vec3,
}

#[derive(Component, Default)]
pub struct BoidsSeparation {
    pub force: Vec3,
}

#[derive(Component, Default)]
pub struct BoidsAlignment {
    pub force: Vec3,
}

#[derive(Component, Default)]
pub struct DesiredVelocity {
    pub force: Vec3,
}

#[derive(Component, Default)]
pub struct WorldBoundForce {
    pub force: Vec3,
}

pub fn move_system(
    mut query: Query<(&mut Transform, &mut Movement, &BoidsCoherence, &BoidsSeparation, &BoidsAlignment, &WorldBoundForce, &DesiredVelocity)>,
    rules: Res<GameRules>,
    boid_rules: Res<BoidsRules>,
    debug: Res<DebugConfig>,
    time: Res<Time>
) {
    if debug.freeze_world { return; }
    for (mut tf, mut mov, coh, sep, ali, bound, des) in &mut query {
        let steering_force = coh.force + sep.force + ali.force + bound.force + des.force;

        // Clamp max acceleration
        if steering_force.length() > boid_rules.max_force {
            steering_force / steering_force.length() * boid_rules.max_force;
        }

        // Apply acceleration to velocity
        mov.vel += steering_force * time.delta_seconds();

        // Clamp velocity
        let max_vel = 125.0;
        if mov.vel.length() > max_vel {
            mov.vel = mov.vel / mov.vel.length() * max_vel;
        }

        tf.translation = tf.translation + mov.vel * time.delta_seconds();
    }
}

pub fn coherence_system(
    mut query: Query<(Entity, &Transform, &Movement, &mut BoidsCoherence)>,
    list: Query<(Entity, &Transform), With<Boid>>,
    res: Res<BoidsRules>,
) {
    for (ent, tf, mov, mut boid) in &mut query {
        let mut count = 0;
        let mut vec = vec3(0.0, 0.0, 0.0);

        for (other_ent, other_tf) in &list {
            if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance(tf.translation);
            if distance < res.perception_range {
                vec += other_tf.translation;
                count += 1;
            }
        }

        // Adds the accumulated pressure to movement component
        match count {
            0 => {
                boid.force = Vec3::ZERO;
            },
            _ => {
                let mut steering = vec / count as f32;
                steering = steering - tf.translation;
                boid.force = steering * res.coherence_factor;
            },
        }
    }
}

pub fn separation_system(
    mut query: Query<(Entity, &Transform, &mut BoidsSeparation)>,
    list: Query<(Entity, &Transform), With<Boid>>,
    rules: Res<BoidsRules>,
) {
    for (ent, tf, mut boid) in &mut query {
        let mut vec = vec3(0.0, 0.0, 0.0);
        let mut count = 0;

        for (tf_ent, other_tf) in &list {
            if ent == tf_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance_squared(tf.translation);
            if distance <= pow(rules.desired_separation, 2) {
                let diff = other_tf.translation - tf.translation;
                let unit_diff = diff / diff.length();
                let pressure = unit_diff * (rules.desired_separation / diff.length());

                vec = vec - pressure;
                count += 1;
            }
        }

        if count > 0 {
            vec = vec / count as f32;
        }

        // Adds the accumulated pressure to movement component
        boid.force = vec * rules.separation_factor;
    }
}

pub fn alignment_system(
    mut query: Query<(Entity, &Transform, &Movement, &mut BoidsAlignment)>,
    list: Query<(Entity, &Transform, &Movement), With<Boid>>,
    rules: Res<BoidsRules>,
) {
    for (ent, tf, mov, mut ali) in &mut query {
        let mut vel = vec3(0.0, 0.0, 0.0);
        let mut count = 0;

        for (other_ent, other_tf, other_mov) in &list {
            if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance_squared(tf.translation);
            if distance <= f32::pow(rules.perception_range, 2) {
                vel += other_mov.vel;
                count += 1;
            }
        }

        match count {
            0 => {
                ali.force = Vec3::ZERO;
            },
            _ => {
                let mut average_vel = vel / count as f32;
                ali.force = (average_vel - mov.vel) * rules.alignment_factor;
            },
        }
    }
}

pub fn desired_velocity_system(
    mut query: Query<(&Movement, &mut DesiredVelocity)>,
    rules: Res<BoidsRules>,
) {
    for (mov, mut des) in &mut query {
        let delta_vel = rules.desired_speed - mov.vel.length();
        let unit_vel = mov.vel / mov.vel.length();

        if !unit_vel.is_nan(){
            des.force = unit_vel * delta_vel * rules.velocity_match_factor; // Should maybe multiply by some configurable constant
        }

    }
}

