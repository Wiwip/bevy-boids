use crate::flock::SteeringPressure;
use crate::perception::Perception;
use crate::physics::{find_nearest_point_on_collider, find_obstacles_in_range};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct ObstacleAvoidance {
    pub factor: f32,
}

pub fn obstacle_avoidance_system(
    query: Query<(
        &Transform,
        &Perception,
        &ObstacleAvoidance,
        &SteeringPressure,
    )>,
    rapier: Res<RapierContext>,
) {
    for (tf, perception, avoid, pressure) in &query {
        let entities = find_obstacles_in_range(&rapier, perception.range, tf.translation);
        let force = obstacle_avoid_steering(&rapier, tf.translation, &entities) * avoid.factor;

        let mut vec = pressure.lock.write().unwrap();
        *vec += force;

        // Only for debug, but broken for now
        //for e in entities {
        //    let _pt = find_nearest_point_on_collider(&rapier, tf.translation.truncate(), e);
        //  lines.line_colored(tf.translation, vec3(pt.x, pt.y, 0.0), 0.0, Color::RED);
        //}
    }
}

fn obstacle_avoid_steering(
    context: &Res<RapierContext>,
    actor_pos: Vec3,
    entities: &Vec<Entity>,
) -> Vec3 {
    let mut count = 0;

    let mut steer: Vec2 = entities
        .into_iter()
        .map(|e| {
            count += 1;

            let point = find_nearest_point_on_collider(context, actor_pos.truncate(), *e);
            let separation = -1.0 * (point - actor_pos.truncate());

            separation
        })
        .sum();

    if count > 0 {
        steer = steer / count as f32;
    }

    let steering = vec3(steer.x, steer.y, 0.0);
    return steering;
}
