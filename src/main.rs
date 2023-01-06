mod debug_systems;
mod boids;
mod helper;
mod interface;

use std::ops::{Div, Sub};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use rand::Rng;
use bevy_prototype_debug_lines::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::f32;
use std::f32::consts::PI;
use crate::boids::{alignment_system, BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, coherence_system, desired_velocity_system, DesiredVelocity, GameRules, move_system, Movement, Particle, separation_system, WorldBound};
use crate::debug_systems::{Dbg, debug_cohesion, debug_lines, debug_print, debug_separation, DebugConfig, DebugSeparation, forces_debug, mouse_track, MouseTracking};
use crate::helper::velocity_angle;

fn main() {
    static DEBUG: &str = "debug";

    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
         // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<BoidsRules>::new())
        .add_plugin(InspectorPlugin::<DebugConfig>::new())
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
            stay_inside: 1.0,
            desired_speed: 75.0,
            max_force: 8.0,
            velocity_match_factor: 0.05,
        })
        .add_system(move_system)
        .add_system(boundaries_system)
        .add_system(rotation_system)

        // Boids Systems
        .add_system(
            separation_system
                .before(move_system)
        )
        .add_system(
            alignment_system
                .before(move_system)
        )
        .add_system(
            coherence_system
                .before(move_system)
        )
        .add_system(
            desired_velocity_system
                .before(move_system)
        )

        // Debug systems
        .insert_resource(DebugConfig {
            debug_location: vec3(-500.0, 400.0, 0.0),
            debug_vector_mag: 2.0,
            display_separation_sum: true,
            display_separation: true,
            display_cohesion: true,
            display_alignment: true,
            ..default()
        })
        .add_stage_after(CoreStage::Update, DEBUG, SystemStage::single_threaded())
        .add_system_to_stage(DEBUG, debug_separation)
        .add_system_to_stage(DEBUG, debug_lines)
        .add_system_to_stage(DEBUG, forces_debug)
        .add_system_to_stage(DEBUG, mouse_track)
        .add_system_to_stage(DEBUG, debug_print)
        .run();
}

struct SpawnedEvent(Entity);

fn add_particles(count: u32) -> Vec<Movement> {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Movement> = Vec::new();

    for _ in 0..count {
        let angle = Quat::from_rotation_z(rng.gen_range(0.0..2.0 * PI));
        let velocity = angle.mul_vec3(Vec3::X) * 150.0;
        let _ = &particles.push(Movement { vel: velocity });
    }
    println!("--------------->>> {}", particles.len());
    particles
}

///
/// Setup the world
///
fn setup(mut commands: Commands, rules: Res<GameRules>) {
    let mut rng = rand::thread_rng();
    let mut dbg = true;
    let vectors = add_particles(rules.particle_count);

    commands.spawn(Camera2dBundle::default());

    for vel in vectors {
        let mut bundle = SpriteBundle {
            sprite: Sprite {
                color: if dbg { Color::RED } else { Color::BLUE },
                custom_size: Some(Vec2::new(6.0, 2.0)),
                ..default()
            },
            ..default()
        };
        bundle.transform.rotation = Quat::from_rotation_z(f32::atan2(vel.vel.y, vel.vel.x));
        bundle.transform.translation.x = rng.gen_range(rules.left..rules.right);
        bundle.transform.translation.y = rng.gen_range(rules.bottom..rules.top);

        let mut ent = commands.spawn((
            bundle,
            Particle,
            vel,
            BoidsCoherence::default(),
            BoidsSeparation::default(),
            BoidsAlignment::default(),
            DesiredVelocity::default(),
            WorldBound::default()
        ));

        if dbg {
            ent.insert(Dbg);
            ent.insert(DebugSeparation);
            ent.insert(MouseTracking);
            dbg = false;
        }
    }
}

fn spawn_debug_particles(mut commands: Commands) {
    hand_spawn_particle(&mut commands, vec2(0.0, 0.0), true);
}

fn hand_spawn_particle(
    commands: &mut Commands,
    position: Vec2,
    debug: bool,
) {
    /*
    let mut bundle = SpriteBundle {
        sprite: Sprite {
            color: if debug { Color::RED } else { Color::BLUE },
            custom_size: Some(Vec2::new(6.0, 2.0)),
            ..default()
        },
        ..default()
    };
    bundle.transform.translation.x = position.x;
    bundle.transform.translation.y = position.y;

    let mut handle = commands.spawn(Particle);
    handle.insert((bundle,
                   BoidsCoherence::default(),
                   BoidsSeparation::default(),
                   BoidsAlignment::default(),
                   DesiredVelocity::default(),
                   WorldBound::default()));

    if debug {
        handle.insert(DebugSeparation);
        handle.insert(MouseTracking);
    }
    */
}

fn boundaries_system(mut query: Query<(&Transform, &mut Movement, &mut WorldBound, Option<&Dbg>)>, rules: Res<GameRules>, boids: Res<BoidsRules>, time: Res<Time>) {
    for (tf, mut mov, mut bound, dbg) in &mut query {
        bound.force.x = 0.0;
        if tf.translation.x >= rules.right {
            // Right X bound
            let delta = rules.right - tf.translation.x;
            bound.force.x = delta * boids.stay_inside;
        }
        if tf.translation.x <= rules.left {
            // Left X bound
            let delta = rules.left - tf.translation.x;
            bound.force.x = delta * boids.stay_inside;
        }

        bound.force.y = 0.0;
        if tf.translation.y <= rules.bottom {
            // Lower Y bound
            let delta = rules.bottom - tf.translation.y;
            bound.force.y = delta * boids.stay_inside;
        }
        if tf.translation.y >= rules.top {
            // Top Y bound
            let delta = rules.top - tf.translation.y;
            bound.force.y = delta * boids.stay_inside;
        }
    }
}

fn rotation_system(mut query: Query<(&mut Transform, &mut Movement)>, time: Res<Time>) {
    for (mut tf, mut vel) in &mut query {
        tf.rotation = Quat::from_rotation_z(velocity_angle(&vel.vel));
    }
}

