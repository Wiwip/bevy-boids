extern crate core;

use crate::boid::Boid;
use crate::flock::{boid_integrator_system, SteeringPressure};
use crate::perception::{perception_system, rapier_perception_system, Perception};
use crate::physics::{
    force_application_system, rotation_system, velocity_system, Acceleration, Velocity,
};
use crate::spatial::partition::{spatial_hash_system, SpatialRes};
use bevy::math::ivec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use spatial::index_partition::IndexPartition;

pub mod behaviours;
pub mod boid;
pub mod flock;
pub mod interface;
pub mod perception;
pub mod physics;
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

pub struct SteeringPlugin;

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spatial_hash_system.before(perception_system))
            .add_system(perception_system.before(BoidStage::ForceCalculation))
            .add_system(rapier_perception_system.before(BoidStage::ForceCalculation))
            .add_system(rotation_system);

        app.add_systems((boid_integrator_system,).in_set(BoidStage::ForceIntegration));
        app.add_systems(
            (force_application_system, velocity_system)
                .chain()
                .in_set(BoidStage::ForceApplication),
        );

        // Ordering of force calculation sets
        app.configure_set(BoidStage::ForceCalculation.before(BoidStage::ForceIntegration));
        app.configure_set(BoidStage::ForceIntegration.before(BoidStage::ForceApplication));

        // Rapier mandatory data
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::ZERO,
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
                    cell_size: 64.0,
                }),
            });
    }
}

#[derive(Bundle)]
pub struct BaseFlockBundle {
    pub boid: Boid,
    pub perception: Perception,
    pub vel: Velocity,
    pub acc: Acceleration,
    pub mesh: SceneBundle,
    pub integrator: SteeringPressure,
}
impl Default for BaseFlockBundle {
    fn default() -> Self {
        Self {
            boid: Boid::default(),
            perception: Perception::default(),
            vel: Velocity::default(),
            acc: Acceleration::default(),
            mesh: Default::default(),
            integrator: SteeringPressure::default(),
        }
    }
}
