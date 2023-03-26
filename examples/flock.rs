extern crate bevy;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{ivec3, vec2};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_flock::physics::{rotation_system};
use bevy_flock::{flock, FlockingPlugin};
use bevy_flock::camera_control::{camera_drag, camera_zoom};
use bevy_flock::debug_systems::{BoidsDebugTools, DebugBoid};
use bevy_flock::flock::{BoidsRules, GameArea};
use bevy_flock::spatial::{spatial_hash_system, SpatialRes};
use bevy_flock::spatial::voxel::VoxelSpace;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlockingPlugin)
        .add_plugin(BoidsDebugTools)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(800.0, 600.0))
        })

        .insert_resource(BoidsRules {
            desired_separation: 20.0,
            desired_speed: 175.0,
            max_force: 1000.0,
            max_velocity: 225.0,
        })

        //.insert_resource(RTreeStorage::default())
        .insert_resource(SpatialRes {
            space: Box::new(VoxelSpace {
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
                cell_size: 32.0,
            })})
        .add_startup_system(setup)


        .add_system(rotation_system)
        .add_system(spatial_hash_system)
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
    let list = flock::new(4000, rules.area, perception);
    commands.spawn_batch(list);

    let debug = flock::new(1, rules.area, perception);
    for i in debug {
        commands.spawn(i)
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(perception).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            })
            .insert(DebugBoid::default());
    }
}
