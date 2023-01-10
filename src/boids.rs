use bevy::math::vec3;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand_distr::num_traits::{Pow, pow};
use crate::boundaries_system;
use crate::physics::{move_system, Spatial};


pub struct BoidsSimulation;

impl Plugin for BoidsSimulation {
    fn build(&self, app: &mut App) {
        app.add_stage_after(CoreStage::Update, BoidStage::ForceCalculation, SystemStage::parallel())
            .add_stage_after(BoidStage::ForceCalculation, BoidStage::ForceIntegration, SystemStage::parallel())
            .add_stage_after(BoidStage::ForceIntegration, BoidStage::ForceApplication, SystemStage::parallel())

            .add_system_set_to_stage(
                BoidStage::ForceCalculation,
                SystemSet::new()
                    .with_system(separation_system)
                    .with_system(alignment_system)
                    .with_system(coherence_system)
                    .with_system(desired_velocity_system)
                    .with_system(boundaries_system),
            )
            .add_system_set_to_stage(
                BoidStage::ForceIntegration,
                SystemSet::new()
                    .with_system(boid_integrator_system::<BoidsCoherence>)
                    .with_system(boid_integrator_system::<BoidsAlignment>)
                    .with_system(boid_integrator_system::<BoidsSeparation>)
                    .with_system(boid_integrator_system::<WorldBoundForce>)
                    .with_system(boid_integrator_system::<DesiredVelocity>),
            )
            .add_system_to_stage(BoidStage::ForceApplication, move_system);
    }
}

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
    pub velocity_match_factor: f32,
    pub freeze_world: bool,
}

#[derive(Resource)]
pub struct GameRules {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub particle_count: u32,
}

pub trait BoidForce {
    fn get_force(&self) -> Vec3;
}

#[derive(Component, Inspectable, Clone, Copy, Default)]
pub struct Movement {
    pub vel: Vec3,
    pub acc: Vec3,
}

#[derive(Component)]
pub struct Boid;

#[derive(Component, Default)]
pub struct BoidsCoherence {
    pub force: Vec3,
}

impl BoidForce for BoidsCoherence {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct BoidsSeparation {
    pub force: Vec3,
}

impl BoidForce for BoidsSeparation {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct BoidsAlignment {
    pub force: Vec3,
}

impl BoidForce for BoidsAlignment {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct DesiredVelocity {
    pub force: Vec3,
}

impl BoidForce for DesiredVelocity {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct WorldBoundForce {
    pub force: Vec3,
}

impl BoidForce for WorldBoundForce {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(StageLabel)]
pub enum BoidStage {
    ForceCalculation,
    ForceIntegration,
    ForceApplication,
}

pub fn boid_integrator_system<T: Component + BoidForce>(
    mut query: Query<(&mut Movement, &T)>,
) {
    for (mut mov, cp) in &mut query {
        mov.acc += cp.get_force()
    }
}

pub fn coherence_system(
    mut query: Query<(Entity, &Transform, &mut BoidsCoherence)>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    for (ent, tf, mut coh) in &mut query {
        let mut count = 0;
        let mut vec = vec3(0.0, 0.0, 0.0);

        // Use data from spatial hash instead of all boids
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let local_boid = map.get_nearby_transforms(&map_coord);

        for (other_ent, other_tf, mov) in local_boid {
            if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance(tf.translation);
            if distance < rules.perception_range {
                vec += other_tf.translation;
                count += 1;
            }
        }

        // Adds the accumulated pressure to movement component
        match count {
            0 => {
                coh.force = Vec3::ZERO;
            }
            _ => {
                let mut steering = vec / count as f32;
                steering = steering - tf.translation;
                coh.force = steering * rules.coherence_factor;
            }
        }
    }
}

pub fn separation_system(
    mut query: Query<(Entity, &Transform, &mut BoidsSeparation)>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    for (ent, tf, mut boid) in &mut query {
        let mut vec = vec3(0.0, 0.0, 0.0);
        let mut count = 0;

        // Use data from spatial hash instead of all boids
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let local_boid = map.get_nearby_transforms(&map_coord);

        for (tf_ent, other_tf, mov) in local_boid {
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
    map: Res<Spatial>,
) {
    for (ent, tf, mov, mut ali) in &mut query {
        let mut vel = vec3(0.0, 0.0, 0.0);
        let mut count = 0;

        // Spatial hash fetch nearby boids
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let local_boid = map.get_nearby_transforms(&map_coord);

        for (other_ent, other_tf, other_mov) in local_boid {
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
            }
            _ => {
                let average_vel = vel / count as f32;
                ali.force = (average_vel - mov.vel) * rules.alignment_factor;
            }
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

        if !unit_vel.is_nan() {
            des.force = unit_vel * delta_vel * rules.velocity_match_factor; // Should maybe multiply by some configurable constant
        }
    }
}

