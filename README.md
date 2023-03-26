# bevy-boids

---

Bevy-boids is a simulator supporting hundreds of generic boids based on steering behaviours designed by [Craig Reynolds](https://www.red3d.com/cwr/boids/).

For now, its only fun to look at, but I'm hoping to add more interesting or emergent behaviours.

## QuickStart
All that's required is to add the plugin and spawn the entities

```Rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlockingPlugin) // flocking plugin
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    game: Res<GameArea>,
) {
    commands.spawn(Camera2dBundle::default());

    let perception = 32.;
    let list = flock::new(4000, game.area, perception);
    commands.spawn_batch(list);
}
```

## Performance

---

On my machine, simulations beyond 5,000 boids on a small world start to kill performance which is an upgrade from the 1,000 without spatial data structure.
I understand that compute shaders are a key way to extend to more entities.

The key elements that will affect your performance are the quantity of entities, but also their density and perception ranges. 
By increasing the game area to 20_000 by 16_000 and the entity count to 12_000 (32. perception), I still have 60 fps.

The biggest cost to the simulation are 'what are the boids in range?' which turns to be O(n^2) using the brute force method.
### R-Tree ([rstar crate](https://crates.io/crates/rstar))
The R-Tree is the most expensive to build, but query times for "within(range)" are almost constant.

### Kd-Tree ([kd-tree crate](https://crates.io/crates/kd-tree))
The Kd-Tree is the fastest tree to build, but the query times are so slow compared to the R-Tree that I have to wonder if I made a mistake in the implementation of the queries.

### HashMap (this)
The HashMap is a nice in-between other spatial data structure. Its performance are likely affected by my basic and naive implementation.

### QuadTree
todo!()

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
