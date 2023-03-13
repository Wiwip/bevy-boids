extern crate core;

use bevy::prelude::*;
use crate::boid::{Boid, Perception};
use crate::flock::{alignment_system, boid_integrator_system, BoidsAlignment, BoidsCoherence, BoidsSeparation, BoidStage, boundaries_system, coherence_system, desired_velocity_system, DesiredVelocity, separation_system, WorldBoundForce};
use crate::physics::{Acceleration, force_application_system, ObstacleAvoidance, Velocity, velocity_system};

pub mod boid;
pub mod debug_systems;
pub mod physics;
pub mod predator;
pub mod interface;
pub mod flock;

pub fn velocity_angle(vel: &Vec3) -> f32 {
    f32::atan2(vel.y, vel.x)
}

pub struct FlockingPlugin;

impl Plugin for FlockingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                separation_system,
                alignment_system,
                coherence_system,
                desired_velocity_system,
                boundaries_system,
            )
                .in_set(BoidStage::ForceCalculation),
        );

        app.add_systems(
            (
                boid_integrator_system::<BoidsCoherence>,
                boid_integrator_system::<BoidsAlignment>,
                boid_integrator_system::<BoidsSeparation>,
                boid_integrator_system::<WorldBoundForce>,
                boid_integrator_system::<DesiredVelocity>,
                boid_integrator_system::<ObstacleAvoidance>,
            )
                .in_set(BoidStage::ForceIntegration),
        );
        app.add_systems(
            (force_application_system, velocity_system)
                .chain()
                .in_set(BoidStage::ForceApplication),
        );

        app.configure_set(BoidStage::ForceCalculation.before(BoidStage::ForceIntegration));
        app.configure_set(BoidStage::ForceIntegration.before(BoidStage::ForceApplication));
    }
}


#[derive(Bundle)]
pub struct BaseFlockBundle {
    pub boid: Boid,
    pub perception: Perception,

    pub desi: DesiredVelocity,
    pub vel: Velocity,
    pub acc: Acceleration,

    pub sp: SpriteBundle,

    pub coh: BoidsCoherence,
    pub sep: BoidsSeparation,
    pub ali: BoidsAlignment,

    pub bounds: WorldBoundForce,
    pub avoid: ObstacleAvoidance,
}

impl Default for BaseFlockBundle {
    fn default() -> Self {
        Self {
            boid: Boid::default(),
            perception: Perception::default(),
            vel: Velocity::default(),
            acc: Acceleration::default(),
            sp: Default::default(),
            coh: Default::default(),
            sep: Default::default(),
            ali: Default::default(),
            desi: Default::default(),
            bounds: Default::default(),
            avoid: Default::default(),
        }
    }
}
