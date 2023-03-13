extern crate bevy;

use bevy::math::{ivec3, vec2};
use bevy::prelude::*;
use bevy_flock::physics::{rotation_system, spatial_hash_system, Spatial};
use bevy_flock::{flock, FlockingPlugin};
use bevy_flock::flock::{BoidsRules, GameArea};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlockingPlugin)

        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(600.0, 400.0))
        })

        .insert_resource(BoidsRules {
            perception_range: 32.0,
            desired_separation: 20.0,
            desired_speed: 175.0,
            max_force: 1000.0,
            max_velocity: 225.0,
        })

        .insert_resource(Spatial {
            map: Default::default(),
            list_offsets: vec![
                ivec3(-1, 1, 0),
                ivec3(0, 1, 0),
                ivec3(1, 1, 0),
                ivec3(-1, 0, 0),
                ivec3(0, 0, 0),
                ivec3(1, 0, 0),
                ivec3(-1, -1, 0),
                ivec3(0, -1, 0),
                ivec3(1, -1, 0),
            ],
            ..default()
        })
        .add_startup_system(setup)

        .add_system(rotation_system)
        .add_system(spatial_hash_system)
        .run();
}

///
/// Setup the world
///
fn setup(
    mut commands: Commands,
    rules: Res<GameArea>
) {
    commands.spawn(Camera2dBundle::default());

    let list = flock::new(500, rules.area, 32.0);
    commands.spawn_batch(list);

}
