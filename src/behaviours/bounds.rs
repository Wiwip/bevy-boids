use crate::flock::{GameArea, SteeringPressure};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct WorldBound {
    pub factor: f32,
}

pub fn boundaries_system(
    mut query: Query<(&Transform, &WorldBound, &SteeringPressure)>,
    rules: Res<GameArea>,
) {
    for (tf, bound, steer) in &mut query {
        let mut force = Vec3::ZERO;
        if tf.translation.x >= rules.area.max_x {
            // Right X bound
            let delta = rules.area.max_x - tf.translation.x;
            force.x = delta * bound.factor;
        } else if tf.translation.x <= rules.area.min_x {
            // Left X bound
            let delta = rules.area.min_x - tf.translation.x;
            force.x = delta * bound.factor;
        }

        if tf.translation.y <= rules.area.min_y {
            //.bottom {
            // Lower Y bound
            let delta = rules.area.min_y - tf.translation.y;
            force.y = delta * bound.factor;
        } else if tf.translation.y >= rules.area.max_y {
            //.top {
            // Top Y bound
            let delta = rules.area.max_y - tf.translation.y;
            force.y = delta * bound.factor;
        }

        if tf.translation.z <= rules.area.min_z {
            //.bottom {
            // Lower Z bound
            let delta = rules.area.min_z - tf.translation.z;
            force.z = delta * bound.factor;
        } else if tf.translation.z >= rules.area.max_z {
            //.top {
            // Top Z bound
            let delta = rules.area.max_z - tf.translation.z;
            force.z = delta * bound.factor;
        }

        if force != Vec3::ZERO {
            let mut vec = steer.lock.write().unwrap();
            *vec += force;
        }
    }
}
