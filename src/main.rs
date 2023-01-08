mod debug_systems;
mod boids;
mod helper;
mod interface;
mod physics;

use std::ops::{BitAnd, Div, Sub};
use bevy::math::vec2;
use bevy::prelude::*;
use rand::Rng;
use bevy_prototype_debug_lines::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable};
use std::f32;
use std::f32::consts::PI;
use crate::boids::{BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, DesiredVelocity, GameRules, Movement, Boid, WorldBoundForce, BoidsSimulation};
use crate::debug_systems::{BoidsDebugTools, DebugBoid};
use crate::physics::rotation_system;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(BoidsDebugTools)
        .add_plugin(BoidsSimulation)
        .add_plugin(InspectorPlugin::<BoidsRules>::new())

        .insert_resource(GameRules {
            left: -1400.0 / 2.0,
            right: 1400.0 / 2.0,
            top: 600.0 / 2.0,
            bottom: -600.0 / 2.0,
            particle_count: 1000,
        })
        .insert_resource(BoidsRules {
            perception_range: 100.0,
            desired_separation: 64.0,
            coherence_factor: 0.08,
            alignment_factor: 0.125,
            separation_factor: 4.0,
            stay_inside: 2.0,
            desired_speed: 75.0,
            max_force: 8.0,
            velocity_match_factor: 0.05,
        })

        .add_system(rotation_system)
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

fn boundaries_system(
    mut query: Query<(&Transform, &mut WorldBoundForce)>,
    rules: Res<GameRules>,
    boids: Res<BoidsRules>
) {
    for (tf, mut bound) in &mut query {
        bound.force = Vec3::ZERO;

        if tf.translation.x >= rules.right {
            // Right X bound
            let delta = rules.right - tf.translation.x;
            bound.force.x = delta * boids.stay_inside;
        } else if tf.translation.x <= rules.left {
            // Left X bound
            let delta = rules.left - tf.translation.x;
            bound.force.x = delta * boids.stay_inside;
        }

        if tf.translation.y <= rules.bottom {
            // Lower Y bound
            let delta = rules.bottom - tf.translation.y;
            bound.force.y = delta * boids.stay_inside;
        } else if tf.translation.y >= rules.top {
            // Top Y bound
            let delta = rules.top - tf.translation.y;
            bound.force.y = delta * boids.stay_inside;
        }
    }
}

