extern crate bevy;

use std::f32;
use std::f32::consts::PI;
use std::ops::{BitAnd, Div, Sub};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{ivec3, vec2};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_prototype_debug_lines::*;
use rand::Rng;
use flock_sim::boids::{Boid, BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, BoidsSimulation, DesiredVelocity, GameRules, Movement, WorldBoundForce};
use flock_sim::physics::{rotation_system, Spatial, spatial_hash_system};

use crate::debug_systems::{BoidsDebugTools, DebugBoid};

mod debug_systems;


fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(BoidsDebugTools)
        .add_plugin(BoidsSimulation)
        .add_plugin(InspectorPlugin::<BoidsRules>::new())

        // FPS Debug
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .insert_resource(GameRules {
            left: -1600.0 / 2.0,
            right: 1600.0 / 2.0,
            top: 800.0 / 2.0,
            bottom: -800.0 / 2.0,
            range: 800.0,
            particle_count: 2000,
        })
        .insert_resource(BoidsRules {
            perception_range: 32.0,
            desired_separation: 8.0,
            coherence_factor: 0.001,
            alignment_factor: 0.0125,
            separation_factor: 0.012,
            stay_inside: 8.0,
            desired_speed: 175.0,
            max_force: 4.0,
            max_velocity: 225.0,
            velocity_match_factor: 0.0,
            freeze_world: false,
        })
        .insert_resource(Spatial{
            map: Default::default(),
            list_offsets: vec![
                ivec3(-1,1,0),ivec3(0,1,0),ivec3(1,1,0),
                ivec3(-1,0,0),ivec3(0,0,0),ivec3(1,0,0),
                ivec3(-1,-1,0),ivec3(0,-1,0),ivec3(1,-1,0)
            ],
            ..default()
        })

        .add_system(rotation_system)
        .add_system(spatial_hash_system)
        .run();
}

///
/// Setup the world
///
fn setup(mut commands: Commands, rules: Res<GameRules>) {
    let mut rng = rand::thread_rng();

    commands.spawn(Camera2dBundle::default());

    let vectors = get_random_boids(rules.particle_count);
    for mov in vectors {
        let position = vec2(rng.gen_range(rules.left..rules.right), rng.gen_range(rules.bottom..rules.top));
        let mut sprite_bundle = build_sprite_bundle(position);
        sprite_bundle.transform.rotation = Quat::from_rotation_z(f32::atan2(mov.vel.y, mov.vel.x));

        commands.spawn((
            sprite_bundle,
            Boid,
            mov,
            BoidsCoherence::default(),
            BoidsSeparation::default(),
            BoidsAlignment::default(),
            DesiredVelocity::default(),
            WorldBoundForce::default()
        ));
    }

    spawn_debug_particle(&mut commands, vec2(400.0, 0.0),
                         DebugBoid{
                             show_separation: true,
                             color: Color::RED,
                             ..default()});
    spawn_debug_particle(&mut commands, vec2(-400.0, 0.0),
                         DebugBoid {
                             show_cohesion: true,
                             color: Color::GREEN,
                             ..default()});
    spawn_debug_particle(&mut commands, vec2(0.0, 200.0),
                         DebugBoid {
                             show_alignment: true,
                             color: Color::VIOLET,
                             ..default()
                         });
    spawn_debug_particle(&mut commands, vec2(0.0, 0.0),
                         DebugBoid {
                             show_cohesion: true,
                             show_separation: true,
                             show_alignment: true,
                             track_mouse: true,
                             show_perception_range: true,
                             spatial_hash: true,
                             color: Color::BLACK,
                             ..default()
                         });
}

fn spawn_debug_particle(
    commands: &mut Commands,
    position: Vec2,
    debug_bundle: impl Bundle,
) {
    let sprite = build_sprite_bundle(position);

    commands.spawn((
        sprite,
        Boid,
        debug_bundle,
        Movement::default(),
        BoidsCoherence::default(),
        BoidsSeparation::default(),
        BoidsAlignment::default(),
        DesiredVelocity::default(),
        WorldBoundForce::default()));
}

fn build_sprite_bundle(
    location: Vec2,
) -> SpriteBundle {
    let mut sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(6.0, 2.0)),
            color: Color::BLUE,
            ..default()
        },
        ..default()
    };
    sprite_bundle.transform.translation.x = location.x;
    sprite_bundle.transform.translation.y = location.y;
    return sprite_bundle;
}

fn get_random_boids(count: u32) -> Vec<Movement> {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Movement> = Vec::new();

    for _ in 0..count {
        let angle = Quat::from_rotation_z(rng.gen_range(0.0..2.0 * PI));
        let velocity = angle.mul_vec3(Vec3::X) * 75.0;
        let _ = &particles.push(Movement { vel: velocity, acc: default() });
    }

    particles
}
