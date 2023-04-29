use crate::behaviours::alignment::alignment_system;
use crate::behaviours::bounds::boundaries_system;
use crate::behaviours::coherence::coherence_system;
use crate::behaviours::separation::separation_system;
use crate::behaviours::velocity_adjust::desired_velocity_system;
use crate::BoidStage;
use bevy::prelude::*;

pub mod alignment;
pub mod avoidance;
pub mod bounds;
pub mod coherence;
pub mod separation;
pub mod velocity_adjust;

pub use alignment::Alignment;
pub use bounds::WorldBound;
pub use coherence::Coherence;
pub use separation::Separation;
pub use velocity_adjust::DesiredVelocity;

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
