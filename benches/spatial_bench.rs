
use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use flock_sim::boids::{Boid, BoidsCoherence, BoidsRules, Movement};
use flock_sim::physics::{Spatial, spatial_hash_system};

pub struct Benchmark {
    world: World,
    hash: Box<dyn System<In=(), Out=()>>,
    brute_system: Box<dyn System<In=(), Out=()>>,
    spatial_query_coherence: Box<dyn System<In=(), Out=()>>,
}


impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut rng = rand::thread_rng();

        world.insert_resource(BoidsRules {
            perception_range: 50.0,
            ..default()
        });
        world.insert_resource(Spatial::default());

        for _ in 0..2_000 {
            world.spawn((
                Transform {
                    translation: vec3(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0), 0.0),
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

        let mut smart = IntoSystem::into_system(spatial_coherence);
        smart.initialize(&mut world);
        smart.update_archetype_component_access(&world);

        let mut brute = IntoSystem::into_system(brute_coherence);
        brute.initialize(&mut world);
        brute.update_archetype_component_access(&world);

        Self{
            world,
            hash: Box::new((system)),
            brute_system: Box::new((brute)),
            spatial_query_coherence: Box::new((smart)),
        }
    }

    pub fn run(&mut self) {
        self.hash.run((), &mut self.world);
    }

    pub fn run_coherence_only(&mut self) {
        self.spatial_query_coherence.run((), &mut self.world);
    }

    pub fn run_full_query(&mut self) {
        self.hash.run((), &mut self.world);
        self.spatial_query_coherence.run((), &mut self.world);
    }

    pub fn run_brute(&mut self) {
        self.brute_system.run((), &mut self.world);
    }
}


pub fn spatial_coherence(
    mut query: Query<(Entity, &Transform, &mut BoidsCoherence)>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
) {
    for (ent, tf, mut coh) in &mut query {
        let mut count = 0;
        let mut vec = vec3(0.0, 0.0, 0.0);

        // Use data from spatial hash instead of all boids
        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let local_boid = map.get_nearby_transforms(&map_coord);

        for (other_ent, other_tf, mov) in local_boid {
            if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance(tf.translation);
            if distance < rules.perception_range {
                vec += other_tf.translation;
                count += 1;
            }
        }

        // Adds the accumulated pressure to movement component
        match count {
            0 => {
                coh.force = Vec3::ZERO;
            }
            _ => {
                let mut steering = vec / count as f32;
                steering = steering - tf.translation;
                coh.force = steering * rules.coherence_factor;
            }
        }
    }
}

pub fn brute_coherence(
    mut query: Query<(Entity, &Transform, &mut BoidsCoherence)>,
    others: Query<(Entity, &Transform)>,
    rules: Res<BoidsRules>,
) {
    for (ent, tf, mut coh) in &mut query {
        let mut count = 0;
        let mut vec = vec3(0.0, 0.0, 0.0);

        for (other_ent, other_tf) in others.iter() {
            if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance(tf.translation);
            if distance < rules.perception_range {
                vec += other_tf.translation;
                count += 1;
            }
        }

        // Adds the accumulated pressure to movement component
        match count {
            0 => {
                coh.force = Vec3::ZERO;
            }
            _ => {
                let mut steering = vec / count as f32;
                steering = steering - tf.translation;
                coh.force = steering * rules.coherence_factor;
            }
        }
    }
}
