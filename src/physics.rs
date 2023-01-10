use bevy::math::ivec3;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::boids::{Boid, BoidsRules, Movement};
use crate::helper::velocity_angle;

#[derive(Resource)]
pub struct Spatial {
    pub map: HashMap<IVec3, Vec<(Entity, Transform, Movement)>>,
    pub list_offsets: Vec<IVec3>,
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

    pub fn get_nearby_transforms(&self, origin: &IVec3) -> Vec<(Entity, Transform, Movement)> {
        let mut list: Vec<(Entity, Transform, Movement)> = default();

        for offset in &self.list_offsets {
            let key = *origin + *offset;

            if let Some(tfs) = self.map.get(&key) {
                list.extend(tfs);
            }
        }
        return list;
    }
}

pub fn rotation_system(mut query: Query<(&mut Transform, &Movement)>) {
    for (mut tf, mov) in &mut query {
        tf.rotation = Quat::from_rotation_z(velocity_angle(&mov.vel));
    }
}

pub fn move_system(
    mut query: Query<(&mut Transform, &mut Movement)>,
    boid_rules: Res<BoidsRules>,
    time: Res<Time>,
) {
    if boid_rules.freeze_world { return; }
    for (mut tf, mut mov) in &mut query {
        let mut acc = mov.acc;
        // Clamp max acceleration
        if acc.length() > boid_rules.max_force {
            acc = acc / acc.length() * boid_rules.max_force;
        }

        // Apply acceleration changes to velocity.
        mov.vel = mov.vel + acc * time.delta_seconds();

        // Clamp velocity
        let max_vel = 125.0; // TODO move max vel
        if mov.vel.length() > max_vel {
            mov.vel = mov.vel / mov.vel.length() * max_vel;
        }

        tf.translation = tf.translation + mov.vel * time.delta_seconds();
        mov.acc = Vec3::ZERO;
    }
}

pub fn spatial_hash_system(
    query: Query<(Entity, &Transform, &Movement), With<Boid>>,
    boid: Res<BoidsRules>,
    mut hash: ResMut<Spatial>,
) {
    hash.map.clear();
    for (ent, tf, mov) in query.iter() {
        let local = hash.global_to_map_loc(&tf.translation, boid.perception_range);

        match hash.map.get_mut(&local) {
            None => {
                hash.map.insert(local, vec![((ent, *tf, *mov))]);
            }
            Some(curr_vec) => {
                curr_vec.push((ent, *tf, *mov));
            }
        }
    }
}
