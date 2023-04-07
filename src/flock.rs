use std::sync::RwLock;
use bevy::math::vec3;
use std::vec::Vec;

use crate::boid::{Boid};
use crate::BaseFlockBundle;
use bevy::prelude::*;
use rand::Rng;
use crate::perception::Perception;

use crate::physics::{Acceleration, ObstacleAvoidance, Velocity};

#[derive(Resource, Default)]
pub struct BoidsRules {
    pub desired_speed: f32,
    pub max_force: f32,
    pub max_velocity: f32,
}

#[derive(Resource)]
pub struct GameArea {
    pub area: Rect,
}

#[derive(Component, Default)]
pub struct BoidsCoherence {
    pub factor: f32,
}

#[derive(Component, Default)]
pub struct BoidsSeparation {
    pub factor: f32,
    pub distance: f32,
}

#[derive(Component, Default)]
pub struct BoidsAlignment {
    pub factor: f32,
}

#[derive(Component, Default)]
pub struct DesiredVelocity {
    pub factor: f32,
}

#[derive(Component, Default)]
pub struct WorldBoundForce {
    pub factor: f32,
}

#[derive(Component, Default)]
pub struct SteeringPressure {
    pub lock: RwLock<Vec3>,
}

pub fn boid_integrator_system(
    mut query: Query<(&mut Acceleration, &SteeringPressure)>
) {
    for (mut acc, steer) in &mut query {
        let force = steer.lock.read().unwrap();
        acc.vec += *force;
        drop(force);

        let mut force = steer.lock.write().unwrap();
        *force = Vec3::ZERO;
        drop(force);
    }
}

pub fn new(count: u32, rect: Rect, perception: f32) -> Vec<BaseFlockBundle> {
    let mut flock = Vec::new();

    for _ in 0..count {
        let bdl = BaseFlockBundle {
            boid: Boid {
                color: Color::BLACK,
            },
            perception: Perception { range: perception, list: vec![] },
            vel: Velocity {
                vec: random_direction(),
            },
            acc: Default::default(),
            sp: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(4.0, 2.0)),
                    color: Color::BLACK,
                    ..default()
                },
                transform: random_transform(rect),
                visibility: Visibility::Visible,
                ..default()
            },
            desi: DesiredVelocity { factor: 1.0 },
            coh: BoidsCoherence { factor: 8.0 },
            sep: BoidsSeparation {
                factor: 4.0,
                distance: 12.0,
            },
            ali: BoidsAlignment { factor: 1.0 },
            bounds: WorldBoundForce { factor: 4.0 },
            avoid: ObstacleAvoidance { factor: 100.0 },
            steer: Default::default(),
        };

        flock.push(bdl);
    }
    flock
}

fn random_transform(rect: Rect) -> Transform {
    let mut rng = rand::thread_rng();

    // Get random position within provided bounds
    let pos = vec3(
        rng.gen_range(rect.min.x..rect.max.x),
        rng.gen_range(rect.min.y..rect.max.y),
        0.0,
    );

    // Create and return transform component
    Transform {
        translation: pos,
        ..default()
    }
}

fn random_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let pos = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
    pos
}

pub fn coherence_system(
    query: Query<(Entity, &Perception, &BoidsCoherence, &SteeringPressure)>,
    boids: Query<&Transform>,
) {
    for (entity, per, coh, steer) in query.iter() {
        let neighbours = &per.list;
        let force = measure_coherence(entity, &boids, neighbours) * coh.factor;

        let mut vec = steer.lock.write().unwrap();
        *vec += force;

    }
}

pub fn measure_coherence(
    entity: Entity,
    query: &Query<&Transform>,
    neighbours: &Vec<Entity>,
) -> Vec3 {
    let local_tf = query.get(entity).unwrap();
    let mut count = 0;

    let steer: Vec3 = neighbours
        .into_iter()
        .map(|&e| {
            if e == entity {
                return Vec3::ZERO;
            }
            let tf = query.get(e).unwrap();
            count += 1;
            tf.translation
        })
        .sum();

    return if count == 0 {
        Vec3::ZERO
    } else {
        (steer / count as f32) - local_tf.translation
    };
}

pub fn separation_system(
    query: Query<(Entity, &Perception, &BoidsSeparation, &SteeringPressure)>,
    boids: Query<&Transform>,
) {
    for (entity, per, sep, steer) in query.iter() {
        // Use data from spatial hash instead of all boids
        let neighbours = &per.list;
        let force = measure_separation(entity, &boids, neighbours, sep.distance) * sep.factor;
        let mut vec = steer.lock.write().unwrap();
        *vec += force;
    }
}

pub fn measure_separation(
    entity: Entity,
    query: &Query<&Transform>,
    neighbours: &Vec<Entity>,
    dist: f32,
) -> Vec3 {
    let mut count = 0;
    let local_tf = query.get(entity).unwrap().translation;

    let result = neighbours
        .into_iter()
        // Exclude our current boid
        .filter(|&&e| entity != e)
        // Get all translations
        .map(|&e| query.get(e).unwrap().translation)
        .map(|v| {
            count += 1;
            let sep = -1.0 * (v - local_tf);
            sep / sep.length() * dist
        })
        .sum();

    return result;
}

pub fn alignment_system(
    query: Query<(Entity, &Perception, &BoidsAlignment, &SteeringPressure)>,
    boids: Query<(&Transform, &Velocity)>,
) {
    for (entity, per, ali, steer) in &query {
        let neighbours = &per.list;
        let force = measure_alignment(entity, &boids, neighbours) * ali.factor;

        let mut vec = steer.lock.write().unwrap();
        *vec += force;
    }

}

pub fn measure_alignment(
    entity: Entity,
    query: &Query<(&Transform, &Velocity)>,
    neighbours: &Vec<Entity>,
) -> Vec3 {
    let mut count = 0;
    let (_, local_mov) = query.get(entity).unwrap();

    let steer: Vec3 = neighbours
        .into_iter()
        .filter(|&e| entity != *e)
        // Get transforms and movement components
        .map(|e| query.get(*e).unwrap())
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
    query: Query<(&Velocity, &DesiredVelocity, &SteeringPressure)>,
    rules: Res<BoidsRules>,
) {
    for (vel, des, steer) in &query {
        let delta_vel = rules.desired_speed - vel.vec.length();
        let unit_vel = vel.vec / vel.vec.length();

        if !unit_vel.is_nan() {
            let force = unit_vel * delta_vel * des.factor;
            let mut vec = steer.lock.write().unwrap();
            *vec += force;
        }
    }
}

pub fn boundaries_system(
    mut query: Query<(&Transform, &WorldBoundForce, &SteeringPressure)>,
    rules: Res<GameArea>,

) {
    for (tf, bound, steer) in &mut query {
        let mut force = Vec3::ZERO;
        if tf.translation.x >= rules.area.max.x {
            // Right X bound
            let delta = rules.area.max.x - tf.translation.x;
            force.x = delta * bound.factor;
        } else if tf.translation.x <= rules.area.min.x {
            // Left X bound
            let delta = rules.area.min.x - tf.translation.x;
            force.x = delta * bound.factor;
        }

        if tf.translation.y <= rules.area.min.y {
            //.bottom {
            // Lower Y bound
            let delta = rules.area.min.y - tf.translation.y;
            force.y = delta * bound.factor;
        } else if tf.translation.y >= rules.area.max.y {
            //.top {
            // Top Y bound
            let delta = rules.area.max.y - tf.translation.y;
            force.y = delta * bound.factor;
        }

        if force != Vec3::ZERO {
            let mut vec = steer.lock.write().unwrap();
            *vec += force;
        }
    }
}
