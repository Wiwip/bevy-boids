use std::ops::Mul;

use bevy::math::ivec3;
use bevy::prelude::*;
use bevy::utils::hashbrown::hash_map::Entry;
use bevy::utils::HashMap;

use crate::boids::{Boid, BoidsRules};
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

pub fn velocity_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
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
