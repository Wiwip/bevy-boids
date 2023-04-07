extern crate bevy;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{ivec3, vec2};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_flock::camera_control::{camera_drag, camera_zoom};
use bevy_flock::debug_systems::{BoidsDebugTools, DebugBoid};
use bevy_flock::flock::{BoidsRules, GameArea};
use bevy_flock::{flock, FlockingPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlockingPlugin)
        .add_plugin(BoidsDebugTools)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(5000.0, 4000.0)),
        })
        .insert_resource(BoidsRules {
            desired_speed: 175.0,
            max_force: 1000.0,
            max_velocity: 225.0,
        })
        .add_startup_system(setup)
        .add_system(camera_drag)
        .add_system(camera_zoom)
        .run();
}

/// Setup the world
///
fn setup(
    mut commands: Commands,
    rules: Res<GameArea>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let perception = 32.;
    let list = flock::new(10000, rules.area, perception);
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
}
