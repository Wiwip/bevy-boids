use bevy::math::vec3;
use bevy::prelude::*;
use flock_sim::boids::{BoidsCoherence, BoidsRules, Movement};
use flock_sim::physics::{move_system, Spatial};
use rand::Rng;

pub struct Benchmark(World, Box<dyn System<In = (), Out = ()>>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut rng = rand::thread_rng();

        world.insert_resource(BoidsRules {
            max_force: 12.0,
            ..default()
        });
        world.insert_resource(Time::default());

        for _ in 0..1_000 {
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
            ));
        }
        let mut system = IntoSystem::into_system(move_system);
        system.initialize(&mut world);
        system.update_archetype_component_access(&world);

        Self(world, Box::new(system))
    }

    pub fn run(&mut self) {
        self.1.run((), &mut self.0);
    }
}
