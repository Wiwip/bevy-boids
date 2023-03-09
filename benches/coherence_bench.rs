use bevy::math::vec3;
use bevy::prelude::*;
use flock_sim::boids::{measure_coherence, Boid, BoidsCoherence, BoidsRules, Movement};
use flock_sim::physics::{spatial_hash_system, Spatial};
use rand::Rng;

pub struct Benchmark {
    world: World,
    hash: Box<dyn System<In = (), Out = ()>>,
    forloop: Box<dyn System<In = (), Out = ()>>,
    iterator: Box<dyn System<In = (), Out = ()>>,
    lazy: Box<dyn System<In = (), Out = ()>>,
}

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut rng = rand::thread_rng();

        world.insert_resource(BoidsRules {
            perception_range: 250.0,
            ..default()
        });
        world.insert_resource(Spatial::default());

        for _ in 0..2_000 {
            world.spawn((
                Transform {
                    translation: vec3(
                        rng.gen_range(-1000.0..1000.0),
                        rng.gen_range(-1000.0..1000.0),
                        0.0,
                    ),
                    rotation: Default::default(),
                    scale: Default::default(),
                },
                Movement {
                    vel: vec3(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0), 0.0),
                    acc: Default::default(),
                },
                Boid,
                BoidsCoherence::default(),
            ));
        }
        let mut system = IntoSystem::into_system(spatial_hash_system);
        system.initialize(&mut world);
        system.update_archetype_component_access(&world);

        let mut foreach = IntoSystem::into_system(coherence_for);
        foreach.initialize(&mut world);
        foreach.update_archetype_component_access(&world);

        let mut iterator = IntoSystem::into_system(coherence_map);
        iterator.initialize(&mut world);
        iterator.update_archetype_component_access(&world);

        let mut lazy = IntoSystem::into_system(lazy_coherence);
        lazy.initialize(&mut world);
        lazy.update_archetype_component_access(&world);

        Self {
            world,
            hash: Box::new(system),
            forloop: Box::new(foreach),
            iterator: Box::new(iterator),
            lazy: Box::new(lazy),
        }
    }

    pub fn run_hash_system(&mut self) {
        self.hash.run((), &mut self.world);
    }

    pub fn run_forloop(&mut self) {
        self.forloop.run((), &mut self.world);
    }

    pub fn run_iterator(&mut self) {
        self.iterator.run((), &mut self.world);
    }

    pub fn run_lazy(&mut self) {
        self.lazy.run((), &mut self.world);
    }
}

pub fn coherence_for(
    mut query: Query<(Entity, &Transform, &mut BoidsCoherence)>,
    boids: Query<(Entity, &Transform)>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    assert_eq!(query.iter().count(), 2_000);

    for (ent, tf, mut coh) in &mut query {
        let mut count = 0;
        let mut vec = vec3(0.0, 0.0, 0.0);

        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        for neighbour in neighbours {
            let (other_ent, other_tf) = boids.get(neighbour).unwrap();
            if ent == other_ent {
                continue;
            } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance(tf.translation);
            if distance < rules.perception_range {
                vec += other_tf.translation;
                count += 1;
            }
        }

        coh.force += if count >= 0 {
            let mut steering = vec / count as f32;
            (steering - tf.translation) * rules.coherence_factor
        } else {
            Vec3::ZERO
        };
        assert_ne!(coh.force, Vec3::ZERO);
    }
}

pub fn coherence_map(
    mut query: Query<(Entity, &Transform, &mut BoidsCoherence)>,
    boids: Query<&Transform>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    assert_eq!(query.iter().count(), 2_000);

    for (ent, tf, mut coh) in &mut query {
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        coh.force += measure_coherence(ent, &boids, neighbours, rules.perception_range)
            * rules.coherence_factor;
    }
}

pub fn lazy_coherence(
    mut query: Query<(Entity, &Transform, &mut BoidsCoherence)>,
    boids: Query<(Entity, &Transform)>,
    rules: Res<BoidsRules>,
) {
    assert_eq!(query.iter().count(), 2_000);

    for (ent, tf, mut coh) in &mut query {
        let mut count = 0;
        let mut vec = vec3(0.0, 0.0, 0.0);

        for (other_ent, other_tf) in &boids {
            if ent == other_ent {
                continue;
            } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance(tf.translation);
            if distance < rules.perception_range {
                vec += other_tf.translation;
                count += 1;
            }
        }

        coh.force += if count >= 0 {
            let mut steering = vec / count as f32;
            (steering - tf.translation) * rules.coherence_factor
        } else {
            Vec3::ZERO
        };
    }
}
