use bevy::prelude::*;
use kd_tree::KdMap;
use bevy_flock::boid::{Boid, Perception};
use bevy_flock::flock::{BoidsCoherence, measure_coherence};
use bevy_flock::spatial::{KdTreeSpace, spatial_hash_system, SpatialPartition};


pub struct Benchmark {
    world: World,
    partition_system: Box<dyn System<In = (), Out = ()>>,
    query_system: Box<dyn System<In = (), Out = ()>>,
}

impl Benchmark {
    pub fn new(pos: Vec<Vec3>) -> Self {
        let mut world = World::new();

        // Insert spatial data structure to be tested
        world.insert_resource(KdTreeSpace {
            tree: KdMap::build_by_ordered_float(Vec::new()),
        });

        // Spawn boids for measurements
        let mut batch = Vec::new();
        for p in pos {
            batch.push((
                    Transform{
                    translation: p,
                    ..default()
                },
                 Perception::default(),
                 BoidsCoherence::default(),
                Boid,
            ))
        }
        world.spawn_batch(batch);

        // Store information in a spatial storage
        let mut partition = IntoSystem::into_system(spatial_hash_system::<KdTreeSpace>);
        partition.initialize(&mut world);
        partition.update_archetype_component_access(&world);

        // Query the information
        let mut query = IntoSystem::into_system(query_system);
        query.initialize(&mut world);
        query.update_archetype_component_access(&world);

        Self {
            world,
            partition_system: Box::new(partition),
            query_system: Box::new(query),
        }
    }

    pub fn run(&mut self) {
        self.partition_system.run((), &mut self.world);
        self.query_system.run((), &mut self.world);
    }

}

fn query_system(
    mut query: Query<(Entity, &Transform, &Perception, &mut BoidsCoherence)>,
    boids: Query<&Transform>,
    map: Res<KdTreeSpace>,
) {
    for (ent, tf, per, mut coh) in &mut query {
        let neighbours = map.get_nearby_ent(&tf.translation, per.range);
        coh.force = measure_coherence(ent, &boids, neighbours) * coh.factor;
    }
}
