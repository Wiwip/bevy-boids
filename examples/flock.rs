extern crate bevy;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_flock::behaviours::avoidance::ObstacleAvoidance;
use bevy_flock::behaviours::{
    Alignment, BoidsPlugin, Coherence, DesiredVelocity, Separation, WorldBound,
};
use bevy_flock::boid::Boid;
use bevy_flock::debug_systems::{BoidsDebugTools, DebugBoid};
use bevy_flock::flock::{random_direction, random_transform, BoidsRules, GameArea};
use bevy_flock::perception::Perception;
use bevy_flock::physics::Velocity;
use bevy_flock::{BaseFlockBundle, SteeringPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(SteeringPlugin)
        .add_plugin(BoidsPlugin)
        .add_plugin(BoidsDebugTools)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(GameArea {
            offset: Vec3::ZERO,
            area: shape::Box::new(1400.0, 1000.0, 1000.0),
        })
        .insert_resource(BoidsRules {
            desired_speed: 175.0,
            max_force: 1000.0,
            max_velocity: 225.0,
        })
        .add_startup_system(setup)
        // .add_system(camera_drag)
        // .add_system(camera_zoom)
        .run();
}

/// Setup the world
fn setup(
    mut commands: Commands,
    rules: Res<GameArea>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0., 200., 1800.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCam);

    let perception = 128.0;
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
                distance: 4.0,
            })
            .insert(Alignment { factor: 2.0 })
            .insert(WorldBound { factor: 4.0 })
            .insert(ObstacleAvoidance { factor: 50.0 })
            .insert(DesiredVelocity { factor: 0.1 });
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(2000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform {
            translation: vec3(0., -600., 0.),
            //rotation: Quat::from_rotation_x(0.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });


    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 50.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, -300.0, 0.0),
        ..default()
    });

    /* commands
        .spawn(BaseFlockBundle::default())
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(perception).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(DebugBoid::default())
        .insert(Perception {
            range: perception,
            ..default()
        })
        .insert(Coherence { factor: 6.0 })
        .insert(Separation {
            factor: 4.0,
            distance: 10.0,
        })
        .insert(Alignment { factor: 1.0 })
        .insert(WorldBound { factor: 4.0 })
        .insert(ObstacleAvoidance { factor: 50.0 })
        .insert(DesiredVelocity { factor: 1.0 });
    */
}
