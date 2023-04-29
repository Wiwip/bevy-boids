use crate::flock::SteeringPressure;
use crate::perception::Perception;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Coherence {
    pub factor: f32,
}

pub fn coherence_system(
    query: Query<(Entity, &Perception, &Coherence, &SteeringPressure)>,
    boids: Query<&Transform>,
) {
    for (entity, per, coh, steer) in query.iter() {
        let neighbours = &per.list;
        let force = measure_coherence(entity, &boids, neighbours) * coh.factor;

        let mut vec = steer.lock.write().unwrap();
        *vec += force;
    }
}

fn measure_coherence(entity: Entity, query: &Query<&Transform>, neighbours: &Vec<Entity>) -> Vec3 {
    let local_tf = query.get(entity).unwrap();
    let mut count = 0;

    let steer: Vec3 = neighbours
        .into_iter()
        .map(|&e| {
            if e == entity {
                return Vec3::ZERO;
            }
            let tf = query.get(e).unwrap();
            count += 1;
            tf.translation
        })
        .sum();

    return if count == 0 {
        Vec3::ZERO
    } else {
        (steer / count as f32) - local_tf.translation
    };
}
