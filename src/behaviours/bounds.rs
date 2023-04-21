use crate::behaviours::WorldBound;
use crate::flock::{GameArea, SteeringPressure};
use bevy::prelude::*;

pub fn boundaries_system(
    mut query: Query<(&Transform, &WorldBound, &SteeringPressure)>,
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
