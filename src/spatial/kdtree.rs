use bevy::prelude::*;
use kd_tree::KdMap;
use crate::spatial::SpatialPartition;

#[derive(Resource)]
pub struct KdTreeSpace {
    pub tree: KdMap<[f32; 2], Entity>
}

impl SpatialPartition for KdTreeSpace {
    fn get_nearby_ent(&self, origin: &Vec3, perception: f32) -> Vec<Entity> {
        let result = self.tree.within_radius(&[origin.x, origin.y], perception);

        let list: Vec<Entity> = result
            .into_iter()
            .map(|(_, e)| *e)
            .collect();

        return list;
    }

    fn insert(&mut self, _ent: Entity, _position: &Vec3) {

    }

    fn bulk_insert(&mut self, bulk: Vec<(Entity, Vec3)>) {
        let list = bulk
            .into_iter()
            .map(|(e, v)| ([v.x, v.y], e))
            .collect();

        self.tree = KdMap::build_by_ordered_float(list);

    }

    fn clear(&mut self) {

    }
}