use bevy::prelude::*;
use crate::boid::Boid;

pub mod voxel;
pub mod simple_list;
pub mod rtree;
pub mod kdtree;


pub trait SpatialPartition {
    fn get_nearby_ent(&self, origin: &Vec3, perception: f32) -> Vec<Entity>;
    fn insert(&mut self, ent: Entity, position: &Vec3);
    fn bulk_insert(&mut self, bulk: Vec<(Entity, Vec3)>);
    fn clear(&mut self);
}

#[derive(Resource)]
pub struct SpatialRes {
    pub space: Box<dyn SpatialPartition + Send + Sync>
}

/// The system is meant to gather all the boids so they can be stored in a space data structure
/// for efficient retrieval at a later date
///
/// The resource must implement the SpatialPartition trait
pub fn spatial_hash_system(
    query: Query<(Entity, &Transform), With<Boid>>,
    mut res: ResMut<SpatialRes>,
) {
    res.space.clear();

    let list = query.into_iter()
        .map(|(e, &tf)| (e, tf.translation))
        .collect();

    res.space.bulk_insert(list);
}