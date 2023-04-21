extern crate bevy;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_flock::behaviours::{Coherence, Separation};
use bevy_flock::camera_control::{camera_drag, camera_zoom};
use bevy_flock::debug_systems::{BoidsDebugTools, DebugBoid};
use bevy_flock::flock::{
    BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, DesiredVelocity, GameArea,
    WorldBoundForce,
};
use bevy_flock::perception::Perception;
use bevy_flock::physics::{obstacle_avoidance_system, ObstacleAvoidance};
use bevy_flock::{flock, BaseFlockBundle, SteeringPlugin};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(SteeringPlugin)
        .add_plugin(BoidsDebugTools)
        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(3000.0, 1200.0)),
        })
        .insert_resource(BoidsRules {
            desired_speed: 75.0,
            max_force: 800.0,
            max_velocity: 125.0,
        })
        .add_system(camera_drag)
        .add_system(camera_zoom)
        .run();
}

///
/// Setup the world
///
fn setup(
    mut commands: Commands,
    rules: Res<GameArea>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let perception = 32.0;

    flock::new(&mut commands, 10000, rules.area, |ec| {
        ec.insert(Perception {
            range: perception,
            ..default()
        })
        .insert(Coherence { factor: 4.0 })
        .insert(Separation {
            factor: 8.0,
            distance: 10.0,
        })
        .insert(BoidsAlignment { factor: 2.0 })
        .insert(WorldBoundForce { factor: 4.0 })
        .insert(ObstacleAvoidance { factor: 50.0 })
        .insert(DesiredVelocity { factor: 1.0 });
    });

    commands
        .spawn(BaseFlockBundle::default())
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(perception).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(DebugBoid::default())
        .insert(Perception {
            range: 32.0,
            list: vec![],
        })
        .insert(BoidsCoherence { factor: 6.0 })
        .insert(BoidsSeparation {
            factor: 4.0,
            distance: 10.0,
        })
        .insert(BoidsAlignment { factor: 1.0 })
        .insert(WorldBoundForce { factor: 4.0 })
        .insert(ObstacleAvoidance { factor: 25.0 })
        .insert(DesiredVelocity { factor: 1.0 });

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
