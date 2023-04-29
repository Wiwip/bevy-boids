use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use std::sync::RwLock;

use crate::physics::Acceleration;

#[derive(Resource, Default)]
pub struct BoidsRules {
    pub desired_speed: f32,
    pub max_force: f32,
    pub max_velocity: f32,
}

#[derive(Resource)]
pub struct GameArea {
    pub offset: Vec3,
    pub area: shape::Box,
}

#[derive(Component, Default)]
pub struct SteeringPressure {
    pub lock: RwLock<Vec3>,
}

pub fn boid_integrator_system(mut query: Query<(&mut Acceleration, &SteeringPressure)>) {
    for (mut acc, steer) in &mut query {
        let force = steer.lock.read().unwrap();
        acc.vec += *force;
        drop(force);

        let mut force = steer.lock.write().unwrap();
        *force = Vec3::ZERO;
        drop(force);
    }
}

pub fn random_transform(area: shape::Box) -> Transform {
    let mut rng = rand::thread_rng();

    // Get random position within provided bounds
    let pos = vec3(
        rng.gen_range(area.min_x..area.max_x),
        rng.gen_range(area.min_y..area.max_y),
        rng.gen_range(area.min_z..area.max_z),
    );

    // Create and return transform component
    Transform {
        translation: pos,
        ..default()
    }
}

pub fn random_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let pos = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
    pos
}
