extern crate bevy;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_flock::camera_control::{camera_drag, camera_zoom};
use bevy_flock::debug_systems::{BoidsDebugTools, DebugBoid};
use bevy_flock::flock::{BoidsRules, GameArea};
use bevy_flock::physics::obstacle_avoidance_system;
use bevy_flock::{flock, FlockingPlugin};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(FlockingPlugin)
        .add_plugin(BoidsDebugTools)

        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(3000.0, 1200.0)),
        })
        .insert_resource(BoidsRules {
            desired_speed: 75.0,
            max_force: 800.0,
            max_velocity: 125.0,
        })

        .add_system(obstacle_avoidance_system)
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

    let perception = 38.;
    let list = flock::new(5000, rules.area, perception);
    commands.spawn_batch(list);

    let debug = flock::new(1, rules.area, perception);
    for i in debug {
        commands
            .spawn(i)
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(perception).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            })
            .insert(DebugBoid::default());
    }

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
