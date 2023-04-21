use crate::behaviours::alignment::alignment_system;
use crate::behaviours::bounds::boundaries_system;
use crate::behaviours::coherence::coherence_system;
use crate::behaviours::separation::separation_system;
use crate::behaviours::velocity::desired_velocity_system;
use crate::BoidStage;
use bevy::prelude::*;

pub mod alignment;
pub mod avoidance;
pub mod bounds;
pub mod coherence;
pub mod separation;
pub mod velocity;

#[derive(Component, Default)]
pub struct Alignment {
    pub factor: f32,
}

#[derive(Component, Default)]
pub struct Coherence {
    pub factor: f32,
}

#[derive(Component, Default)]
pub struct Separation {
    pub factor: f32,
    pub distance: f32,
}

#[derive(Component, Default)]
pub struct DesiredVelocity {
    pub factor: f32,
}

#[derive(Component, Default)]
pub struct WorldBound {
    pub factor: f32,
}

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
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
    }
}
