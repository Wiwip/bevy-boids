# bevy-boids

---

Bevy-boids is an example of a boids simulation from steering behaviours as designed by [Craig Reynolds](https://www.red3d.com/cwr/boids/).

https://user-images.githubusercontent.com/5385314/235312447-15393d30-3173-4792-874c-4e4b4981d2d7.mp4

## QuickStart
Add the plugin and spawn the entities

```Rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SteeringPlugin) // flocking plugin
        .add_plugin(BoidsPlugin) // boids plugin to adjust behaviours
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    game: Res<GameArea>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(50., 100., 150.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });

    let perception = 10.0;
    let count = 1000;

    for _ in 0..count {
        let boid = BaseFlockBundle {
            boid: Boid {
                color: Color::BLACK,
            },
            perception: Default::default(),
            vel: Velocity {
                vec: random_direction(),
            },
            acc: Default::default(),
            mesh: SceneBundle {
                scene: asset_server.load("models/bird.gltf#Scene0"),
                transform: random_transform(rules.area),
                ..default()
            },
            integrator: Default::default(),
        };

        commands
            .spawn(boid)
            .insert(Perception {
                range: perception,
                ..default()
            })
            .insert(Coherence { factor: 4.0 })
            .insert(Separation {
                factor: 8.0,
                distance: 0.75,
            })
            .insert(Alignment { factor: 2.0 })
            .insert(WorldBound { factor: 4.0 })
            .insert(ObstacleAvoidance { factor: 50.0 })
            .insert(DesiredVelocity { factor: 0.1 });
    }
}
```

## Performance

---

On my machine, simulations beyond 5,000 boids on a small world start to kill performance which is an upgrade from the 1,000 without spatial data structure.
I understand that compute shaders are a key way to extend to more entities.


## Desired Features
### General
- [x] Flocking behavour (coherence, separation, alignment)
- [x] Constant speed
- [X] Obstacle avoidance (through rapier2D obstacles)
- [ ] Add steering toward point
- [ ] Environmental effects (wind or currents)

### Performance
- [x] SpatialHash lookup (supports kd-tree, r-tree, hashmap, and bruteforce lists)
- [ ] Choose one and be done with it

### Debug
- [ ] Display force vectors
- [ ] Display nearby boids

### Possible Features
- [ ] Predators
- [ ] Reproduction
- [ ] Evolutionary

# License

---

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:
- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
