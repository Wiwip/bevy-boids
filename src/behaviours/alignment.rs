use crate::behaviours::Alignment;
use crate::flock::SteeringPressure;
use crate::perception::Perception;
use crate::physics::Velocity;
use bevy::prelude::*;

pub fn alignment_system(
    query: Query<(Entity, &Perception, &Alignment, &SteeringPressure)>,
    boids: Query<(&Transform, &Velocity)>,
) {
    for (entity, per, ali, steer) in &query {
        let neighbours = &per.list;
        let force = measure_alignment(entity, &boids, neighbours) * ali.factor;

        let mut vec = steer.lock.write().unwrap();
        *vec += force;
    }
}

fn measure_alignment(
    entity: Entity,
    query: &Query<(&Transform, &Velocity)>,
    neighbours: &Vec<Entity>,
) -> Vec3 {
    let mut count = 0;
    let (_, local_mov) = query.get(entity).unwrap();

    let steer: Vec3 = neighbours
        .into_iter()
        .filter(|&e| entity != *e)
        // Get transforms and movement components
        .map(|e| query.get(*e).unwrap())
        .map(|(_, &vel)| {
            count += 1;
            return vel.vec;
        })
        .sum();

    return if count == 0 {
        Vec3::ZERO
    } else {
        (steer / count as f32) - local_mov.vec
    };
}
