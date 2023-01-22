extern crate bevy;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use std::f32::consts::PI;
use std::ops::{BitAnd, Div, Sub};
use std::sync::Arc;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::boids::{
    Boid, BoidsAlignment, BoidsBoundaries, BoidsCoherence, BoidsRules, BoidsSeparation,
    BoidsSimulation, DesiredVelocity, GameRules,
};
use crate::debug_systems::{BoidsDebugTools, DebugBoid};
use crate::physics::rotation_system;

mod boids;
mod debug_systems;
mod helper;
mod interface;
mod physics;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(BoidsSimulation)
        .add_plugin(InspectorPlugin::<BoidsRules>::new())
        // FPS Debug
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Rapier physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        //.add_plugin(BoidsDebugTools)
        .insert_resource(GameRules {
            left: -1800.0 / 2.0,
            right: 1800.0 / 2.0,
            top: 1000.0 / 2.0,
            bottom: -1000.0 / 2.0,
            particle_count: 2000,
        })
        .insert_resource(BoidsRules {
            perception_range: 100.0,
            desired_separation: 90.0,
            coherence_factor: 100.0,
            alignment_factor: 20.0,
            separation_factor: 20.0,
            stay_inside: 100.0,
            desired_speed: 255.0,
            max_force: 8.0,
            velocity_match_factor: 0.05,
            freeze_world: false,
        })
        .add_system(rotation_system)
        .run();
}

///
/// Setup the world
///
fn setup(
    mut commands: Commands,
    rules: Res<GameRules>,
    mut rapier: ResMut<RapierConfiguration>,
    boids_rules: Res<BoidsRules>
) {
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());
    rapier.gravity = Vec2::ZERO;

    let vectors = get_random_boids(rules.particle_count);
    for mov in vectors {
        let position = vec2(
            rng.gen_range(rules.left..rules.right),
            rng.gen_range(rules.bottom..rules.top),
        );
        let mut sprite_bundle = build_sprite_bundle(position);

        let _ = commands
            .spawn((
                sprite_bundle,
                Boid::default(),
                RigidBody::Dynamic,
                Collider::ball(3.0),
                ColliderMassProperties::Density(1.0),
                ExternalForce::default(),
                CollisionGroups::new(
                    Group::from_bits(0b0001).unwrap(),
                    Group::from_bits(0b0010).unwrap(),
                ),
                BoidsBoundaries::default(),
                BoidsCoherence::default(),
                BoidsAlignment::default(),
                BoidsSeparation::default(),
                DesiredVelocity::default(),
                Velocity{
                    linvel: Vec2::new(
                        rng.gen_range(-100.0..100.0),
                        rng.gen_range(-100.0..100.0),
                    ),
                    ..default()
                }
            ))
            .with_children(|child| {
                child.spawn((
                    Collider::ball(boids_rules.perception_range),
                    Sensor,
                    TransformBundle::default(),
                    CollisionGroups::new(
                        Group::from_bits(0b0010).unwrap(),
                        Group::from_bits(0b0001).unwrap(),
                    ),
                ));
            });
    }

    spawn_debug_particle(
        &mut commands,
        vec2(0.0, 0.0),
        DebugBoid {
            show_cohesion: true,
            show_separation: true,
            show_alignment: true,
            track_mouse: true,
            show_perception_range: true,
            spatial_hash: true,
            color: Color::BLACK,
            ..default()
        },
    );
}

fn spawn_debug_particle(commands: &mut Commands, position: Vec2, debug_bundle: impl Bundle) {
    let sprite = build_sprite_bundle(position);

    let _ = commands
        .spawn((
            sprite,
            Boid::default(),
            debug_bundle,
            RigidBody::Dynamic,
            Collider::ball(3.0),
            ColliderMassProperties::Density(1.0),
            ExternalForce::default(),
            CollisionGroups::new(
                Group::from_bits(0b0001).unwrap(),
                Group::from_bits(0b0010).unwrap(),
            ),
            BoidsBoundaries::default(),
            BoidsCoherence::default(),
            BoidsAlignment::default(),
            BoidsSeparation::default(),
            DesiredVelocity::default(),
            Velocity::default(),
        ))
        .with_children(|child| {
            child.spawn((
                Collider::ball(12.0),
                Sensor,
                TransformBundle::default(),
                CollisionGroups::new(
                    Group::from_bits(0b0010).unwrap(),
                    Group::from_bits(0b0001).unwrap(),
                ),
            ));
        });
}

fn build_sprite_bundle(location: Vec2) -> SpriteBundle {
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

fn get_random_boids(count: u32) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Vec3> = Vec::new();

    for _ in 0..count {
        let angle = Quat::from_rotation_z(rng.gen_range(0.0..2.0 * PI));
        let velocity = angle.mul_vec3(Vec3::X) * 75.0;
        let _ = &particles.push(velocity);
    }

    particles
}

fn boundaries_system(
    mut query: Query<(&Transform, &mut BoidsBoundaries)>,
    rules: Res<GameRules>,
    boids: Res<BoidsRules>,
) {
    for (tf, mut boid) in query.iter_mut() {
        if tf.translation.x >= rules.right {
            // Right X bound
            let delta = rules.right - tf.translation.x;
            boid.force.x = delta * boids.stay_inside;
        } else if tf.translation.x <= rules.left {
            // Left X bound
            let delta = rules.left - tf.translation.x;
            boid.force.x = delta * boids.stay_inside;
        } else {
            boid.force.x = 0.0;
        }

        if tf.translation.y <= rules.bottom {
            // Lower Y bound
            let delta = rules.bottom - tf.translation.y;
            boid.force.y = delta * boids.stay_inside;
        } else if tf.translation.y >= rules.top {
            // Top Y bound
            let delta = rules.top - tf.translation.y;
            boid.force.y = delta * boids.stay_inside;
        } else {
            boid.force.y = 0.0;
        }
    }
}
