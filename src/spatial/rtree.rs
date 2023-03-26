use bevy::math::vec3;
use bevy::prelude::*;
use rand_distr::num_traits::pow;
use rstar::{AABB, Point, RTree};
use crate::spatial::SpatialPartition;

#[derive(Clone, PartialEq, Debug)]
struct RTreeEntity {
    entity: Entity,
    vec: Vec3,
}

impl Point for RTreeEntity {
    type Scalar = f32;
    const DIMENSIONS: usize = 3;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        RTreeEntity {
            entity: Entity::PLACEHOLDER,
            vec: vec3(generator(0), generator(1), generator(2)),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.vec.x,
            1 => self.vec.y,
            2 => self.vec.z,
            _ => unreachable!()
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.vec.x,
            1 => &mut self.vec.y,
            2 => &mut self.vec.z,
            _ => unreachable!()
        }
    }
}

#[derive(Resource, Default)]
pub struct RTreeStorage {
    tree: RTree<RTreeEntity>,
}


impl SpatialPartition for RTreeStorage {
    fn get_nearby_ent(&self, origin: &Vec3, perception: f32) -> Vec<Entity> {
        let range_check = AABB::from_corners(
            RTreeEntity {
                entity: Entity::PLACEHOLDER,
                vec: vec3(origin.x - perception, origin.y - perception, 0.0),
            },
            RTreeEntity {
                entity: Entity::PLACEHOLDER,
                vec: vec3(origin.x + perception, origin.y + perception, 0.0),
            });

        let elements = self.tree.locate_within_distance(
            RTreeEntity{
                entity: Entity::PLACEHOLDER,
                vec: *origin,
            }, pow(perception, 2));

        let list = elements.
            map(|r| r.entity)
            .collect();

        return list;
    }

    fn insert(&mut self, ent: Entity, position: &Vec3) {
        self.tree.insert(RTreeEntity{
            entity: ent,
            vec: *position,
        })
    }

    fn bulk_insert(&mut self, bulk: Vec<(Entity, Vec3)>) {
        let mut list = Vec::new();
        for (e, v) in bulk {
            list.push(RTreeEntity {
                entity: e,
                vec: v,
            })
        }

        self.tree = RTree::bulk_load(list);
    }

    fn clear(&mut self) {
        self.tree = RTree::default();
    }
}