extern crate bevy;

use bevy::math::{ivec3, vec2};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_flock::debug_systems::DebugBoid;
use bevy_flock::physics::{
    obstacle_avoidance_system, rotation_system, spatial_hash_system, ObstacleAvoidance, Spatial,
    Velocity,
};
use rand::Rng;
use std::f32;
use std::f32::consts::PI;
use bevy_flock::{BaseFlockBundle, flock, FlockingPlugin};
use bevy_flock::flock::{BoidsRules, GameArea};


fn main() {
    App::new()
        .add_startup_system(setup)
        //.insert_resource(ClearColor(Color::rgb(0.8, 0.4, 0.2))
        .add_plugins(DefaultPlugins)
        // .add_plugin(DebugLinesPlugin::default())
        .add_plugin(FlockingPlugin)
        //  .add_plugin(InspectorPlugin::<BoidsRules>::new())
        // Rapier 2D
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        // FPS Debug
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(1000.0, 600.0))
        })
        .insert_resource(BoidsRules {
            perception_range: 32.0,
            desired_separation: 20.0,
       //     coherence_factor: 8.0,
       //     alignment_factor: 8.0,
       //     separation_factor: 8.0,
        //    stay_inside: 10.0,
            desired_speed: 175.0,
            max_force: 1000.0,
            max_velocity: 225.0,
         //   velocity_match_factor: 2.00,
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
        .add_system(rotation_system)
        .add_system(spatial_hash_system)
        .add_system(obstacle_avoidance_system)
        .run();
}

///
/// Setup the world
///
fn setup(mut commands: Commands, rules: Res<GameArea>) {
    commands.spawn(Camera2dBundle::default());

    let list = flock::new(5000, rules.area, 32.0);
    commands.spawn_batch(list);

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(50.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, 200.0, 0.0,
        )));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::triangle(
            vec2(25.0, 0.0),
            vec2(-25.0, 0.0),
            vec2(0.0, 75.0),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -200.0, -150.0, 0.0,
        )));
}
