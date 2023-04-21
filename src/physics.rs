use crate::flock::BoidsRules;
use crate::velocity_angle;
use bevy::ecs::entity::Entity;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand_distr::weighted_alias::AliasableWeight;
use std::ops::Mul;

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

pub fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut tf, vel) in &mut query {
        tf.translation += vel.vec * time.delta_seconds();
    }
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
pub fn find_obstacles_in_range(
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
pub fn find_nearest_point_on_collider(
    context: &Res<RapierContext>,
    from_position: Vec2,
    target_entity: Entity,
) -> Vec2 {
    let binding = |e| e == target_entity;
    let filter = QueryFilter::default().predicate(&binding);

    let (_, pp) = context.project_point(from_position, true, filter).unwrap();
    return pp.point;
}
