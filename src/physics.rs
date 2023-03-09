use std::ops::Mul;

use bevy::ecs::entity::Entity;
use bevy::math::{ivec3, vec3};
use bevy::prelude::*;
use bevy::reflect::Array;
use bevy::utils::hashbrown::hash_map::Entry;
use bevy::utils::HashMap;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;
use rand_distr::weighted_alias::AliasableWeight;

use crate::boids::{Boid, BoidForce, BoidsRules};
use crate::velocity_angle;

#[derive(Component, Copy, Clone, Default)]
pub struct Velocity {
    pub vec: Vec3,
}

#[derive(Component, Copy, Clone, Default)]
pub struct Acceleration {
    pub vec: Vec3,
}

#[derive(Resource, Default)]
pub struct Spatial {
    pub map: HashMap<IVec3, Vec<Entity>>,
    pub list_offsets: Vec<IVec3>,
    pub cell_size: f32,
}

impl Spatial {
    pub fn global_to_map_loc(&self, global: &Vec3, cell_size: f32) -> IVec3 {
        let mut pos = *global / cell_size;
        pos.x = f32::floor(pos.x);
        pos.y = f32::floor(pos.y);
        pos.z = f32::floor(pos.z);
        let tpl = ivec3(pos.x as i32, pos.y as i32, pos.z as i32);
        return tpl;
    }

    /// Get a list of Entity that are considered nearby by the spatial hashing algorithm
    ///
    /// # Arguments
    ///
    /// * `origin`: The coordinate of the location where to start looking from
    ///
    /// returns: Vec<Entity>
    pub fn get_nearby_ent(&self, origin: &IVec3) -> Vec<Entity> {
        let mut list: Vec<Entity> = default();

        for offset in &self.list_offsets {
            let key = *origin + *offset;

            if let Some(tfs) = self.map.get(&key) {
                list.extend(tfs);
            }
        }
        return list;
    }
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

pub fn spatial_hash_system(
    query: Query<(Entity, &Transform), With<Boid>>,
    rules: Res<BoidsRules>,
    mut hash: ResMut<Spatial>,
) {
    hash.map.clear();
    hash.cell_size = rules.perception_range;

    for (ent, tf) in query.iter() {
        let local = hash.global_to_map_loc(&tf.translation, rules.perception_range);

        // Add entity to selected map cell
        match hash.map.entry(local) {
            Entry::Occupied(mut o) => {
                o.get_mut().push(ent);
            }
            Entry::Vacant(v) => {
                v.insert(vec![(ent)]);
            }
        };
    }
}

#[derive(Component, Default)]
pub struct ObstacleAvoidance {
    pub force: Vec3,
}

impl BoidForce for ObstacleAvoidance {
    fn get_force(&self) -> Vec3 {
        return self.force;
    }
}

pub fn obstacle_avoidance_system(
    mut query: Query<(Entity, &Transform, &mut ObstacleAvoidance), With<Boid>>,
    rules: Res<BoidsRules>,
    rapier: Res<RapierContext>,
    // mut lines: ResMut<DebugLines>,
) {
    for (_, tf, mut avoid) in query.iter_mut() {
        let entities = find_obstacles_in_range(&rapier, rules.perception_range, tf.translation);
        avoid.force = obstacle_avoid_steering(&rapier, tf.translation, &entities) * 80.0;

        for e in entities {
            let pt = find_nearest_point_on_collider(&rapier, tf.translation.truncate(), e);
            //  lines.line_colored(tf.translation, vec3(pt.x, pt.y, 0.0), 0.0, Color::RED);
        }
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

            let m = separation.length();
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
