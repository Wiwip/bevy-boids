use crate::behaviours::Separation;
use crate::flock::SteeringPressure;
use crate::perception::Perception;
use bevy::prelude::*;

pub fn separation_system(
    query: Query<(Entity, &Perception, &Separation, &SteeringPressure)>,
    boids: Query<&Transform>,
) {
    for (entity, per, sep, steer) in query.iter() {
        // Use data from spatial hash instead of all behaviours
        let neighbours = &per.list;
        let force = measure_separation(entity, &boids, neighbours, sep.distance) * sep.factor;
        let mut vec = steer.lock.write().unwrap();
        *vec += force;
    }
}

pub fn measure_separation(
    entity: Entity,
    query: &Query<&Transform>,
    neighbours: &Vec<Entity>,
    dist: f32,
) -> Vec3 {
    let mut count = 0;
    let local_tf = query.get(entity).unwrap().translation;

    let result = neighbours
        .into_iter()
        // Exclude our current boid
        .filter(|&&e| entity != e)
        // Get all translations
        .map(|&e| query.get(e).unwrap().translation)
        .map(|v| {
            count += 1;
            let sep = -1.0 * (v - local_tf);
            sep / sep.length() * dist
        })
        .sum();

    return result;
}
