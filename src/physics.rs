use std::ops::Mul;

use crate::boid::{Boid};
use bevy::ecs::entity::Entity;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand_distr::weighted_alias::AliasableWeight;

use crate::flock::BoidsRules;
use crate::perception::Perception;
use crate::velocity_angle;

pub struct SteeringEvent {
    pub entity: Entity,
    pub force: Vec3,
}

#[derive(Component, Copy, Clone, Default)]
pub struct Velocity {
    pub vec: Vec3,
}

#[derive(Component, Copy, Clone, Default)]
pub struct Acceleration {
    pub vec: Vec3,
}

pub fn rotation_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut tf, vel) in &mut query {
        tf.rotation = Quat::from_rotation_z(velocity_angle(&vel.vec));
    }
}

pub fn force_application_system(
    mut query: Query<(&mut Velocity, &mut Acceleration)>,
    boid_rules: Res<BoidsRules>,
    time: Res<Time>,
) {
    for (mut vel, mut acc) in &mut query {
        // Clamp max acceleration
        if acc.vec.length() > boid_rules.max_force {
            acc.vec = acc.vec.normalize_or_zero().mul(boid_rules.max_force);
        }

        // Apply acceleration changes to velocity.
        vel.vec += acc.vec * time.delta_seconds();
        acc.vec = Vec3::ZERO;

        // Clamp velocity before releasing to other systems
        if vel.vec.length() > boid_rules.max_velocity {
            vel.vec = vel.vec.normalize_or_zero().mul(boid_rules.max_velocity);
        }
    }
}

pub fn velocity_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
) {
    for (mut tf, vel) in &mut query {
        tf.translation += vel.vec * time.delta_seconds();
    }
}

#[derive(Component, Default)]
pub struct ObstacleAvoidance {
    pub factor: f32,
}

pub fn obstacle_avoidance_system(
    query: Query<(Entity, &Transform, &Perception, &ObstacleAvoidance), With<Boid>>,
    rapier: Res<RapierContext>,
    mut event: EventWriter<SteeringEvent>,
) {
    let mut forces = Vec::new();

    for (entity, tf, perception, avoid) in &query {
        let entities = find_obstacles_in_range(&rapier, perception.range, tf.translation);
        let force = obstacle_avoid_steering(&rapier, tf.translation, &entities) * avoid.factor;

        forces.push(SteeringEvent { entity, force });

        // Only for debug, but broken for now
        for e in entities {
            let _pt = find_nearest_point_on_collider(&rapier, tf.translation.truncate(), e);
            //  lines.line_colored(tf.translation, vec3(pt.x, pt.y, 0.0), 0.0, Color::RED);
        }
    }

    event.send_batch(forces);
}

/// Finds all obstacles in perception range using an intersection with shape in rapier
///
/// # Arguments
///
/// * `context`: Rapier context
/// * `perception`: How far to look
/// * `location`: Where to look from
///
/// returns: Vec<Entity, Global>
fn find_obstacles_in_range(
    context: &Res<RapierContext>,
    perception: f32,
    location: Vec3,
) -> Vec<Entity> {
    let shape = Collider::ball(perception);
    let filter = QueryFilter::default();
    let mut entities = Vec::new();

    context.intersections_with_shape(location.truncate(), Rot::ZERO, &shape, filter, |entity| {
        entities.push(entity);
        true // Continue searching for other colliders
    });
    return entities;
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

/// Finds the closest point to a boid projected onto a shape.
/// Typically used to measure steering pressure by obstacles
///
/// # Arguments
///
/// * `rapier_context`: The rapier context
/// * `from_position`: The position of the boid
/// * `target_collider`: The obstacle in range
///
/// returns: Vec2
fn find_nearest_point_on_collider(
    context: &Res<RapierContext>,
    from_position: Vec2,
    target_entity: Entity,
) -> Vec2 {
    let binding = |e| e == target_entity;
    let filter = QueryFilter::default().predicate(&binding);

    let (_, pp) = context.project_point(from_position, true, filter).unwrap();
    return pp.point;
}
