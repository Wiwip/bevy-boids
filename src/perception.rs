use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::partition::SpatialRes;


#[derive(Component, Default)]
pub struct Perception {
    pub range: f32,
    pub list: Vec<Entity>,
}


pub fn perception_system(
    mut query: Query<(&mut Perception, &Transform)>,
    mut rapier: Res<RapierContext>
) {
    let rotation = 0.0;
    let filter = QueryFilter::default();

    for (mut per, tf) in &mut query {
        let mut list = Vec::new();
        let shape = Collider::ball(per.range);
        let pos = tf.translation.truncate();

        // Clear previously perceived entities
        per.list.clear();

        // Cast shape and add perceived entities to list
        rapier.intersections_with_shape(pos, rotation, &shape, filter, |e|{
           &list.push(e);
            true
        });

        per.list.extend(list);
    }
}

pub fn index_perception_system(
    mut query: Query<(&mut Perception, &Transform)>,
    space: Res<SpatialRes>
) {
    for (mut per, tf) in &mut query {
        let pos = tf.translation;

        // Clear previously perceived entities
        per.list.clear();
        // Cast shape and add perceived entities to list
        let nearby = space.space.get_nearby_ent(&pos, per.range);
        per.list.extend(nearby);
    }
}