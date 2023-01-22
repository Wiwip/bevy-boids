use crate::boids::BoidStage::ForceApplication;
use crate::boundaries_system;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::{ExternalForce, ExternalImpulse, RapierContext, Sensor, Velocity};
use rand_distr::num_traits::Pow;
use std::sync::Arc;
use std::sync::Mutex;

pub struct BoidsSimulation;

impl Plugin for BoidsSimulation {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            BoidStage::ForceCalculation,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BoidStage::ForceCalculation,
            BoidStage::ForceIntegration,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BoidStage::ForceIntegration,
            ForceApplication,
            SystemStage::parallel(),
        )
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
                .with_system(boid_integrator_system::<BoidsBoundaries>)
                .with_system(boid_integrator_system::<DesiredVelocity>),
        )
        .add_system_to_stage(ForceApplication, boid_force_application);
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

#[derive(Component, Default)]
pub struct Boid {
    force: Vec3,
}

#[derive(StageLabel)]
pub enum BoidStage {
    ForceCalculation,
    ForceIntegration,
    ForceApplication,
}

pub trait BoidForce {
    fn get_force(&self) -> Vec3;
}

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
pub struct BoidsBoundaries {
    pub force: Vec3,
}

impl BoidForce for BoidsBoundaries {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

pub fn boid_integrator_system<T: Component + BoidForce>(
    mut query: Query<(&mut Boid, &T)>
) {
    for (mut steer, cp) in &mut query {
        steer.force += cp.get_force()
    }
}

pub fn boid_force_application(
    mut query: Query<(&mut Boid, &mut ExternalForce)>,
    rules: Res<BoidsRules>
) {
    for (mut boid, mut ext) in &mut query {
        if boid.force.length() > rules.max_force {
            ext.force = (boid.force / boid.force.length() * rules.max_force).truncate();
        } else {
            ext.force = boid.force.truncate();
        }
        boid.force = Vec3::ZERO;
    }
}

pub fn coherence_system(
    mut boids: Query<(&Transform, &mut BoidsCoherence)>,
    mut sensors: Query<(Entity, &Sensor, &Parent)>,
    rapier_context: Res<RapierContext>,
    boid_config: Res<BoidsRules>,
) {
    for (sensor_ent, _, parent) in sensors.iter() {
        let aspan = info_span!("A sensor calculations").entered();
        let center_of_mass = measure_center_of_mass(&boids, sensor_ent, &rapier_context);

        // Apply vector to sensor parent
        if let Ok((mtf, mut coh)) = boids.get_mut(parent.get()) {
                let mut steering = center_of_mass;
                steering = steering - mtf.translation;
                coh.force = steering * boid_config.coherence_factor;
        }
    }
}


fn measure_center_of_mass(
    boids: &Query<(&Transform, &mut BoidsCoherence)>,
    sensor_ent: Entity,
    rapier_context: &Res<RapierContext>
) -> Vec3 {
    let mut count = 0;
    let mut center_of_mass = Vec3::ZERO;

    let aspan = info_span!("COM Measurements").entered();

    // Get the intersecting boids with the sensor
    for (ent1, ent2, intersecting) in rapier_context.intersections_with(sensor_ent) {

        let other_ent = if ent1 == sensor_ent { ent2 } else { ent1 };

        if intersecting {
            if let Ok((other_tf, _)) = boids.get(other_ent) {
                center_of_mass += other_tf.translation;
                count += 1;
            }
        }
    }
    if count > 0 {
        return center_of_mass / count as f32;
    } else {
        return Vec3::ZERO;
    }

}


pub fn separation_system(
    mut boids: Query<(&Transform, &Sprite, &mut BoidsSeparation)>,
    mut sensors: Query<(Entity, &Sensor, &Parent, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    boid_config: Res<BoidsRules>,
) {
    for (sensor_ent, _, parent, gtf) in sensors.iter() {
        let mut count = 0;
        let mut vec = Vec3::ZERO;

        // Get the intersecting boids with the sensor
        for (ent1, ent2, intersecting) in rapier_context.intersections_with(sensor_ent) {
            let other_ent = if ent1 == sensor_ent { ent2 } else { ent1 };

            if intersecting {
                if let Ok((other_tf, _, _)) = boids.get(other_ent) {
                    count += 1;
                    let diff = other_tf.translation - gtf.translation();
                    let unit_diff = diff / diff.length();
                    let pressure = unit_diff * (boid_config.desired_separation / diff.length());

                    vec = vec - pressure;
                }
            }
        }

        // Apply vector to sensor parent
        if let Ok((mtf, _, mut sep)) = boids.get_mut(parent.get()) {
            match count {
                0 => sep.force = Vec3::ZERO,
                _ => {
                    vec = vec / count as f32;
                    sep.force = vec * boid_config.separation_factor;
                }
            }
        }
    }
}

pub fn alignment_system(
    mut boids: Query<(&Transform, &Sprite, &mut BoidsAlignment, &Velocity)>,
    mut sensors: Query<(Entity, &Sensor, &Parent, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    boid_config: Res<BoidsRules>,
    mut lines: ResMut<DebugLines>,
) {
    for (sensor_ent, _, parent, gtf) in sensors.iter() {
        let mut count = 0;
        let mut vec = Vec2::ZERO;

        // Get the intersecting boids with the sensor
        for (ent1, ent2, intersecting) in rapier_context.intersections_with(sensor_ent) {
            let aspan = info_span!("PerformIntersectionsAli").entered();
            let other_ent = if ent1 == sensor_ent { ent2 } else { ent1 };

            if intersecting {
                if let Ok((other_tf, _, _, vel)) = boids.get(other_ent) {
                    count += 1;
                    vec = vec + vel.linvel;
                }
            }
        }


        // Apply vector to sensor parent
        if let Ok((tf, _, mut ali, vel)) = boids.get_mut(parent.get()) {
            let aspan = info_span!("ApplyVectorAli").entered();
            match count {
                0 => ali.force = Vec3::ZERO,
                _ => {
                    vec = vec / count as f32; // Averages velocities
                    vec = vec - vel.linvel; // Computes the velocity difference
                    ali.force = vec3(vec.x, vec.y, 0.0) * boid_config.alignment_factor;
                }
            }
        }
    }
}

pub fn desired_velocity_system(
    mut query: Query<(&Velocity, &mut DesiredVelocity)>,
    rules: Res<BoidsRules>) {

    for (vel, mut des) in &mut query {
        let delta_vel = rules.desired_speed - vel.linvel.length();
        let unit_vel = vel.linvel / vel.linvel.length();

        if !unit_vel.is_nan() {
            des.force = vec3(unit_vel.x, unit_vel.y, 0.0) * delta_vel * rules.velocity_match_factor; // Should maybe multiply by some configurable constant
        }
    }
}
