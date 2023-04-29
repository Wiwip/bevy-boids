extern crate bevy;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_flock::behaviours::avoidance::ObstacleAvoidance;
use bevy_flock::behaviours::{
    Alignment, BoidsPlugin, Coherence, DesiredVelocity, Separation, WorldBound,
};
use bevy_flock::boid::Boid;
use bevy_flock::flock::{random_direction, random_transform, BoidsRules, GameArea};
use bevy_flock::perception::Perception;
use bevy_flock::{BaseFlockBundle, SteeringPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(spawn_colliders)
        .add_plugins(DefaultPlugins)
        .add_plugin(SteeringPlugin)
        .add_plugin(BoidsPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(GameArea {
            offset: Vec3::ZERO,
            area: shape::Box::new(100.0, 100.0, 100.0),
        })
        .insert_resource(BoidsRules {
            desired_speed: 100.0,
            max_force: 1000.0,
            max_velocity: 200.0,
        })
        .run();
}

///
/// Setup the world
///
fn setup(
    mut commands: Commands,
    rules: Res<GameArea>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(50., 100., 150.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCam);

    let perception = 12.0;
    let count = 1000;

    for _ in 0..count {
        let boid = BaseFlockBundle {
            boid: Boid {
                color: Color::BLACK,
            },
            perception: Default::default(),
            vel: bevy_flock::physics::Velocity {
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

    /*
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(400.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform {
            translation: vec3(0., -100., 0.),
            ..default()
        },
        ..default()
    });
    */

    // X
    let transform = Transform::from_xyz(5., -50., 0.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 1.0))),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::BLUE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // Y
    let transform = Transform::from_xyz(0., -45., 0.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 10.0, 1.0))),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // Z
    let transform = Transform::from_xyz(0., -50., 5.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 10.0))),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            first_cascade_far_bound: 250.0,
            maximum_distance: 1000.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn spawn_colliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform = Transform::from_xyz(0., 0., 0.);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 10.0, 10.0))),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::INDIGO,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(Collider::cuboid(5.0, 5.0, 5.0));

    let transform = Transform::from_xyz(-10., 50., 50.);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 4.0,
                depth: 10.0,
                ..default()
            })),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(Collider::capsule_y(5.0, 4.0));
}
