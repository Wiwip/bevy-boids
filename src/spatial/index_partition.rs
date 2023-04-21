use crate::spatial::partition::SpatialPartition;
use bevy::math::ivec3;
use bevy::prelude::*;
use bevy::utils::{Entry, HashMap};
use rand_distr::num_traits::pow;

#[derive(Resource, Default)]
pub struct IndexPartition {
    pub map: HashMap<IVec3, Vec<(Entity, Vec3)>>,
    pub list_offsets: Vec<IVec3>,
    pub cell_size: f32,
}

impl IndexPartition {
    pub fn global_to_map_loc(&self, global: &Vec3) -> IVec3 {
        let mut pos = *global / self.cell_size;
        pos.x = f32::floor(pos.x);
        pos.y = f32::floor(pos.y);
        pos.z = f32::floor(pos.z);
        let tpl = ivec3(pos.x as i32, pos.y as i32, pos.z as i32);
        return tpl;
    }
}

impl SpatialPartition for IndexPartition {
    /// Get a list of Entity that are considered nearby by the spatial hashing algorithm
    ///
    /// # Arguments
    ///
    /// * `origin`: The coordinate of the location where to start looking from
    ///
    /// returns: Vec<Entity>
    fn get_nearby_ent(&self, global: &Vec3, perception: f32) -> Vec<Entity> {
        let mut list = Vec::new();
        let local = self.global_to_map_loc(global);

        // Broad range checks
        for offset in &self.list_offsets {
            let key = local + *offset;

            if let Some(tfs) = self.map.get(&key) {
                list.extend(tfs);
            }
        }

        // Precise range check
        let perception_squared = pow(perception, 2);

        let mut temp_list = Vec::new();
        for (ent, pos) in list {
            if pos.distance_squared(*global) <= perception_squared {
                temp_list.push(ent)
            }
        }
        temp_list
    }

    fn insert(&mut self, entity: Entity, global: &Vec3) {
        let local = self.global_to_map_loc(global);

        // Add entity to selected map cell
        match self.map.entry(local) {
            Entry::Occupied(mut o) => {
                o.get_mut().push((entity, *global));
            }
            Entry::Vacant(v) => {
                v.insert(vec![(entity, *global)]);
            }
        };
    }

    fn bulk_insert(&mut self, bulk: Vec<(Entity, Vec3)>) {
        for (e, p) in bulk {
            self.insert(e, &p);
        }
    }

    fn clear(&mut self) {
        self.map.clear();
    }
}
