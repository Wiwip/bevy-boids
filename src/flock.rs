use std::f32::consts::PI;
use std::vec::Vec;
use bevy::math::{vec2, vec3};

use bevy::prelude::*;
use rand::Rng;
use rand_distr::num_traits::{pow, Pow};
use crate::{BaseFlockBundle};
use crate::boid::{Boid, Perception};

use crate::physics::{
    Acceleration, Spatial, Velocity,
};


#[derive(Resource, Default)]
pub struct BoidsRules {
    pub perception_range: f32,
    pub desired_separation: f32,
    pub desired_speed: f32,
    pub max_force: f32,
    pub max_velocity: f32,
}

#[derive(Resource)]
pub struct GameArea {
    pub area: Rect,
}

pub trait Steering {
    fn get_force(&self) -> Vec3;
}


#[derive(Component, Default)]
pub struct BoidsCoherence {
    pub factor: f32,
    pub force: Vec3,
}

impl Steering for BoidsCoherence {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct BoidsSeparation {
    pub factor: f32,
    pub force: Vec3,
}

impl Steering for BoidsSeparation {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct BoidsAlignment {
    pub factor: f32,
    pub force: Vec3,
}

impl Steering for BoidsAlignment {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct DesiredVelocity {
    pub force: Vec3,
    pub factor: f32,
}

impl Steering for DesiredVelocity {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Component, Default)]
pub struct WorldBoundForce {
    pub factor: f32,
    pub force: Vec3,
}

impl Steering for WorldBoundForce {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum BoidStage {
    ForceCalculation,
    ForceIntegration,
    ForceApplication,
}

pub fn boid_integrator_system<T: Component + Steering>(mut query: Query<(&mut Acceleration, &T)>) {
    for (mut acc, cp) in &mut query {
        acc.vec += cp.get_force()
    }
}

pub fn new(
    count: u32,
    rect: Rect,
    perception: f32
) -> Vec<BaseFlockBundle> {
    let mut flock = Vec::new();

    for _ in 0..count {
        let bdl = BaseFlockBundle {
            boid: Boid,
            perception: Perception { range: perception },
            vel: Velocity {
                vec: random_direction(),
            },
            acc: Default::default(),
            sp: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(6.0, 2.0)),
                    color: Color::BLACK,
                    ..default()
                },
                transform: random_transform(rect),
                visibility: Visibility::Visible,
                ..default()
            },
            desi: DesiredVelocity {
                factor: 1.0,
                ..default()
            },
            coh: BoidsCoherence {
                factor: 8.0,
                ..default()
            },
            sep: BoidsSeparation {
                factor: 8.0,
                ..default()
            },
            ali: BoidsAlignment {
                factor: 8.0,
                ..default()
            },
            bounds: WorldBoundForce {
                factor: 12.0,
                ..default() },
            avoid: Default::default(),
        };

        flock.push(bdl);

    }
    flock
}

fn random_transform(rect: Rect) -> Transform {
    let mut rng = rand::thread_rng();

    // Get random position within provided bounds
    let pos = vec3(
        rng.gen_range(rect.min.x .. rect.max.x),
        rng.gen_range(rect.min.y .. rect.max.y),
        0.0
    );

    // Get random rotation between 0 and 360 degrees
    let rot = Quat::from_rotation_z(rng.gen_range(0.0..PI*2.0));

    // Create and return transform component
    Transform {
        translation: pos,
        ..default()
    }
}

fn random_direction() -> Vec3 {
    let mut rng = rand::thread_rng();

    let pos = vec3(
        rng.gen_range(-1.0 .. 1.0),
        rng.gen_range(-1.0 .. 1.0),
        0.0
    );

    pos
}

pub fn coherence_system(
    mut query: Query<(Entity, &Transform, &Perception, &mut BoidsCoherence)>,
    boids: Query<&Transform>,
    map: Res<Spatial>,
) {
    for (ent, tf, per, mut coh) in &mut query {
        let map_coord = map.global_to_map_loc(&tf.translation, per.range);
        let neighbours = map.get_nearby_ent(&map_coord);

        coh.force = measure_coherence(ent, &boids, neighbours, per.range) * coh.factor;
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

    let steer: Vec3 = neighbours
        .into_iter()
        .map(|e| {
            if e == entity {
                return Vec3::ZERO;
            }
            let tf = query.get(e).unwrap();
            return if tf.translation.distance_squared(local_tf.translation) <= perception_squared {
                count += 1;
                tf.translation
            } else {
                Vec3::ZERO
            };
        })
        .sum();

    return if count == 0 {
        Vec3::ZERO
    } else {
        (steer / count as f32) - local_tf.translation
    };
}

pub fn separation_system(
    mut query: Query<(Entity, &Transform, &mut BoidsSeparation)>,
    boids: Query<&Transform>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    for (ent, tf, mut sep) in &mut query {
        // Use data from spatial hash instead of all boids
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        sep.force = measure_separation(ent, &boids, neighbours, rules.desired_separation) * sep.factor;
    }
}

pub fn measure_separation(
    entity: Entity,
    query: &Query<&Transform>,
    neighbours: Vec<Entity>,
    perception: f32,
) -> Vec3 {
    let mut count = 0;
    let perception_squared = pow(perception, 2);
    let local_tf = query.get(entity).unwrap().translation;

    let result = neighbours
        .into_iter()
        // Exclude our current boid
        .filter(|&e| entity != e)
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
    for (ent, tf, mut ali) in &mut query {
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        ali.force = measure_alignment(ent, &boids, neighbours, rules.perception_range) * ali.factor;
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
        .filter(|(&tf, _)| {
            tf.translation.distance_squared(local_tf.translation) <= perception_squared
        })
        .map(|(_, &vel)| {
            count += 1;
            return vel.vec;
        })
        .sum();

    return if count == 0 {
        Vec3::ZERO
    } else {
        (steer / count as f32) - local_mov.vec
    };
}

pub fn desired_velocity_system(
    mut query: Query<(&Velocity, &mut DesiredVelocity)>,
    rules: Res<BoidsRules>,
) {
    for (vel, mut des) in &mut query {
        let delta_vel = rules.desired_speed - vel.vec.length();
        let unit_vel = vel.vec / vel.vec.length();

        if !unit_vel.is_nan() {
            des.force = unit_vel * delta_vel * des.factor;
        }
    }
}

pub fn boundaries_system(
    mut query: Query<(&Transform, &mut WorldBoundForce)>,
    rules: Res<GameArea>,
) {
    for (tf, mut bound) in &mut query {
        if tf.translation.x >= rules.area.max.x {
            // Right X bound
            let delta = rules.area.max.x - tf.translation.x;
            bound.force.x = delta * bound.factor;
        } else if tf.translation.x <= rules.area.min.x {
            // Left X bound
            let delta = rules.area.min.x - tf.translation.x;
            bound.force.x = delta * bound.factor;
        }

        if tf.translation.y <= rules.area.min.y { //.bottom {
            // Lower Y bound
            let delta = rules.area.min.y - tf.translation.y;
            bound.force.y = delta * bound.factor;
        } else if tf.translation.y >= rules.area.max.y { //.top {
            // Top Y bound
            let delta = rules.area.max.y - tf.translation.y;
            bound.force.y = delta * bound.factor;
        }
    }
}