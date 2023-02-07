use bevy::prelude::*;
use bevy::reflect::Array;
use std::vec::Vec;
use bevy_inspector_egui::Inspectable;
use rand_distr::num_traits::{Pow, pow};

use crate::physics::{Acceleration, move_system, Spatial, Velocity};

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
    pub max_velocity: f32,
    pub velocity_match_factor: f32,
    pub freeze_world: bool,
}

#[derive(Resource)]
pub struct GameRules {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub range: f32,
    pub particle_count: u32,
}

pub trait BoidForce {
    fn get_force(&self) -> Vec3;
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
    mut query: Query<(&mut Velocity, &mut Acceleration, &T)>,
    rules: Res<BoidsRules>,
) {
    if rules.freeze_world { return;}
    for (mut vel, acc, cp) in &mut query {
        vel.vec += cp.get_force()
    }
}

pub fn coherence_system(
    mut query: Query<(Entity, &Transform, &mut BoidsCoherence)>,
    boids: Query<&Transform>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    if rules.freeze_world { return;}
    for (ent, tf, mut coh) in &mut query {

        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        coh.force = measure_coherence(ent, &boids, neighbours, rules.perception_range) * rules.coherence_factor;
    }
}

pub fn measure_coherence(
    entity: Entity,
    query: &Query<&Transform>,
    neighbours: Vec<Entity>,
    perception: f32,
) -> Vec3 {
    let perception_squared = f32::pow(perception, 2);
    let local_tf = query.get(entity).unwrap();
    let mut count = 0;
/*
    let steer: Vec3 = neighbours
        .into_iter()

        // Exclude current boid
        .filter(|e| e != &entity)

        // Get all transforms
        .map(|v| query.get(v).unwrap().translation )

        // Exclude boid outside perception range
        .filter(|v| v.distance_squared(local_tf.translation) <= perception_squared)

        // Count leftovers
        .map(|v| {
            count += 1;
            return v;
        })
        .sum();
*/
    let steer: Vec3 = neighbours
        .into_iter()
        .map(|e|{
            if e == entity { return Vec3::ZERO }
            let tf = query.get(e).unwrap();
            return if tf.translation.distance_squared(local_tf.translation) <= perception_squared {
                count += 1;
                tf.translation
            } else {
                Vec3::ZERO
            }
        })
        .sum();

    return if count == 0 {
        Vec3::ZERO
    } else {
        (steer / count as f32) - local_tf.translation
    }
}

pub fn separation_system(
    mut query: Query<(Entity, &Transform, &mut BoidsSeparation)>,
    boids: Query<(&Transform)>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    if rules.freeze_world { return;}
    for (ent, tf, mut sep) in &mut query {
        // Use data from spatial hash instead of all boids
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        sep.force = measure_separation(ent, &boids, neighbours, rules.desired_separation) * rules.separation_factor;
    }
}

pub fn measure_separation(
    entity: Entity,
    query: &Query<(&Transform)>,
    neighbours: Vec<Entity>,
    perception: f32,
) -> Vec3 {
    let mut count = 0;
    let perception_squared = pow(perception, 2);
    let local_tf = query.get(entity).unwrap().translation;

    let result = neighbours
        .into_iter()

        // Exclude our current boid
        .filter(|&e| entity != e )

        // Get all translations
        .map(|e| query.get(e).unwrap().translation)

        // Filter boids that are too far
        .filter(|v| v.distance_squared(local_tf) <= perception_squared)

        .map(|v| {
            count += 1;
            let sep = -1.0 * (v - local_tf);
            sep / sep.length() * perception
        })

        .sum();

    return result;
}

pub fn alignment_system(
    mut query: Query<(Entity, &Transform, &mut BoidsAlignment)>,
    boids: Query<(&Transform, &Velocity)>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    if rules.freeze_world { return;}
    for (ent, tf, mut ali) in &mut query {
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        ali.force = measure_alignment(ent, &boids, neighbours, rules.perception_range) * rules.alignment_factor;
    }

}

pub fn measure_alignment(
    entity: Entity,
    query: &Query<(&Transform, &Velocity)>,
    neighbours: Vec<Entity>,
    perception: f32,
) -> Vec3 {
    let mut count = 0;
    let (local_tf, local_mov) = query.get(entity).unwrap();
    let perception_squared = pow(perception, 2);

    let steer: Vec3 = neighbours
        .into_iter()
        .filter(|&e| entity != e)

        // Get transforms and movement components
        .map(|e| query.get(e).unwrap())

        // Excludes boids that are too far
        .filter(|(&tf, _)| tf.translation.distance_squared(local_tf.translation) <= perception_squared)

        .map(|(_, &vel)| {
            count += 1;
            return vel.vec;
        })
        .sum();

    return if count == 0 {
        Vec3::ZERO
    } else {
        (steer / count as f32) - local_mov.vec
    }
}

pub fn desired_velocity_system(
    mut query: Query<(&Velocity, &mut DesiredVelocity)>,
    rules: Res<BoidsRules>,
) {
    for (vel, mut des) in &mut query {
        let delta_vel = rules.desired_speed - vel.vec.length();
        let unit_vel = vel.vec / vel.vec.length();

        if !unit_vel.is_nan() {
            des.force = unit_vel * delta_vel * rules.velocity_match_factor;
        }
    }
}


pub fn boundaries_system(
    mut query: Query<(&Transform, &mut WorldBoundForce)>,
    rules: Res<GameRules>,
    boids: Res<BoidsRules>,
) {
    for (tf, mut bound) in &mut query {

        if tf.translation.x >= rules.right {
            // Right X bound
            let delta = rules.right - tf.translation.x;
            bound.force.x = delta * boids.stay_inside;
        } else if tf.translation.x <= rules.left {
            // Left X bound
            let delta = rules.left - tf.translation.x;
            bound.force.x = delta * boids.stay_inside;
        }

        if tf.translation.y <= rules.bottom {
            // Lower Y bound
            let delta = rules.bottom - tf.translation.y;
            bound.force.y = delta * boids.stay_inside;
        } else if tf.translation.y >= rules.top {
            // Top Y bound
            let delta = rules.top - tf.translation.y;
            bound.force.y = delta * boids.stay_inside;
        }

        //bound.force = -tf.translation / rules.range * boids.stay_inside;
    }
}

