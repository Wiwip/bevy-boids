extern crate core;

use crate::boid::Boid;
use crate::flock::{
    alignment_system, boid_integrator_system, BoidsAlignment, BoidsCoherence,
    BoidsSeparation, boundaries_system, coherence_system, desired_velocity_system, DesiredVelocity,
    separation_system, SteeringPressure, WorldBoundForce,
};
use spatial::index_partition::IndexPartition;
use crate::perception::{Perception, perception_system, rapier_perception_system};
use crate::physics::{
    Acceleration, force_application_system, ObstacleAvoidance, rotation_system, Velocity,
    velocity_system,
};
use bevy::math::ivec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::spatial::partition::{spatial_hash_system, SpatialRes};

pub mod boid;
pub mod camera_control;
pub mod debug_systems;
pub mod flock;
pub mod interface;
pub mod perception;
pub mod physics;
pub mod predator;
pub mod spatial;

pub fn velocity_angle(vel: &Vec3) -> f32 {
    f32::atan2(vel.y, vel.x)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum BoidStage {
    ForceCalculation,
    ForceIntegration,
    ForceApplication,
}

pub struct FlockingPlugin;

impl Plugin for FlockingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spatial_hash_system.before(perception_system))
            .add_system(perception_system.before(BoidStage::ForceCalculation))

            .add_system(rapier_perception_system.before(BoidStage::ForceCalculation))
            .add_system(rotation_system);

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
                boid_integrator_system,
            )
                .in_set(BoidStage::ForceIntegration),
        );
        app.add_systems(
            (force_application_system, velocity_system)
                .chain()
                .in_set(BoidStage::ForceApplication),
        );

        // Ordering of force calculation sets
        app.configure_set(BoidStage::ForceCalculation.before(BoidStage::ForceIntegration));
        app.configure_set(BoidStage::ForceIntegration.before(BoidStage::ForceApplication));

        // Rapier mandatory data
        app
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                physics_pipeline_active: false,
                query_pipeline_active: true,
                ..default()
            })

            .insert_resource(SpatialRes {
                space: Box::new(IndexPartition {
                    map: Default::default(),
                    list_offsets: vec![
                        ivec3(-1, 1, 0),
                        ivec3(0, 1, 0),
                        ivec3(1, 1, 0),
                        ivec3(-1, 0, 0),
                        ivec3(0, 0, 0),
                        ivec3(1, 0, 0),
                        ivec3(-1, -1, 0),
                        ivec3(0, -1, 0),
                        ivec3(1, -1, 0),
                    ],
                    cell_size: 38.0,
                }),
            });
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
    pub steer: SteeringPressure,
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
            steer: Default::default(),
        }
    }
}
