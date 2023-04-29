use crate::spatial::partition::SpatialRes;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct Perception {
    pub range: f32,
    pub list: Vec<Entity>,
}

pub fn rapier_perception_system(
    mut query: Query<(&mut Perception, &Transform)>,
    rapier: Res<RapierContext>,
) {
    let filter = QueryFilter::default();

    for (mut per, tf) in &mut query {
        let mut list = Vec::new();
        let shape = Collider::ball(per.range);
        let pos = tf.translation;

        // Clear previously perceived entities
        per.list.clear();

        // Cast shape and add perceived entities to list
        rapier.intersections_with_shape(pos, Rot::default(), &shape, filter, |e| {
            let _ = &list.push(e);
            true
        });

        per.list.extend(list);
    }
}

pub fn perception_system(mut query: Query<(&mut Perception, &Transform)>, space: Res<SpatialRes>) {
    for (mut per, tf) in &mut query {
        let pos = tf.translation;

        // Clear previously perceived entities
        per.list.clear();
        // Cast shape and add perceived entities to list
        let nearby = space.space.get_nearby_ent(&pos, per.range);
        per.list.extend(nearby);
    }
}
