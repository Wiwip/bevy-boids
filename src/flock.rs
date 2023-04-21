use crate::boid::Boid;
use crate::BaseFlockBundle;
use bevy::ecs::system::EntityCommands;
use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use std::sync::RwLock;

use crate::physics::{Acceleration, Velocity};

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

pub fn new(commands: &mut Commands, count: u32, rect: Rect, f: impl Fn(&mut EntityCommands)) {
    for _ in 0..count {
        let boid = BaseFlockBundle {
            boid: Boid {
                color: Color::BLACK,
            },
            perception: Default::default(),
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
            integrator: Default::default(),
        };

        let ec = &mut commands.spawn(boid);

        // Opportunity to add other components to a basic flock
        f(ec);
    }
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
