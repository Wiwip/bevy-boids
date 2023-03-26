use crate::spatial::SpatialPartition;
use bevy::prelude::*;
use rand_distr::num_traits::pow;

#[derive(Resource, Default)]
pub struct BrutePartition {
    entities: Vec<(Entity, Vec3)>,
}

impl SpatialPartition for BrutePartition {
    fn get_nearby_ent(&self, origin: &Vec3, perception: f32) -> Vec<Entity> {
        let perception_squared = pow(perception, 2);

        let mut temp_list = Vec::new();
        for (ent, pos) in &self.entities {
            if pos.distance_squared(*origin) <= perception_squared {
                temp_list.push(*ent)
            }
        }
        temp_list
    }

    fn insert(&mut self, ent: Entity, position: &Vec3) {
        self.entities.push((ent, *position));
    }

    fn bulk_insert(&mut self, bulk: Vec<(Entity, Vec3)>) {
        self.entities.extend(bulk);
    }

    fn clear(&mut self) {
        self.entities.clear();
    }
}
