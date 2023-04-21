use crate::behaviours::{Alignment, Coherence, Separation};
use crate::boid::Boid;
use crate::BoidStage;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::perception::Perception;
use crate::physics::Velocity;

#[derive(Component, Default)]
pub struct DebugBoid {
    pub show_separation: bool,
    pub show_cohesion: bool,
    pub show_alignment: bool,
    pub show_perception_range: bool,
    pub color: Color,
    pub track_mouse: bool,
    pub spatial_hash: bool,
}

#[derive(Reflect, Resource, Default)]
pub struct DebugConfig {
    pub track_mouse: bool,
    pub display_separation_sum: bool,
    pub display_separation: bool,
    pub display_cohesion: bool,
    pub display_alignment: bool,
    pub display_perceived: bool,
    pub velocity_info: bool,
    pub desired_velocity: bool,
    pub display_bound: bool,
    pub debug_location: Vec3,
    pub debug_vector_mag: f32,
}

pub struct BoidsDebugTools;

impl Plugin for BoidsDebugTools {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                debug_separation,
                debug_cohesion,
                debug_alignment,
                debug_perception_range,
                debug_world_bounds,
                mouse_track,
            )
                .after(BoidStage::ForceIntegration),
        )
        .add_system(reset_color_system.before(debug_tag_spatial_hash_system))
        //.add_plugin(ResourceInspectorPlugin::<DebugConfig>::new())
        .insert_resource(DebugConfig {
            debug_location: vec3(-500.0, 400.0, 0.0),
            debug_vector_mag: 1.0,
            display_separation_sum: true,
            display_separation: true,
            display_cohesion: true,
            display_alignment: true,
            display_perceived: true,
            ..default()
        });
    }
}

pub fn debug_world_bounds(
    //query: Query<(&Transform, &WorldBoundForce, &DebugBoid)>,
    //mut lines: ResMut<DebugLines>,
    //rules: Res<GameArea>,
    config: Res<DebugConfig>,
) {
    if !config.display_bound {
        return;
    }

    /* let start = vec3(rules.left, rules.top, 0.0);
    let end = vec3(rules.right, rules.top, 0.0);
    let duration = 0.0; // Duration of 0 will show the line for 1 frame.
    //  lines.line_colored(start, end, duration, Color::BLACK);

    let start = vec3(rules.left, rules.bottom, 0.0);
    let end = vec3(rules.right, rules.bottom, 0.0);
    let duration = 0.0; // Duration of 0 will show the line for 1 frame.
    //  lines.line_colored(start, end, duration, Color::BLACK);

    for (tf, bound, _) in query.iter() {
        //    lines.line_colored(tf.translation, tf.translation + bound.force, duration, Color::CYAN);
    }
    */
}

pub fn debug_cohesion(
    query: Query<(Entity, &Transform, &Coherence, &DebugBoid)>,
    //behaviours: Query<&Transform>,
    //rules: Res<BoidsRules>,
    //map: Res<VoxelSpace>,
) {
    for (_ent, _tf, _, debug) in query.iter() {
        // Display only for debug_cohesion enabled behaviours
        if !debug.show_cohesion {
            continue;
        }

        //let map_coord = map.global_to_map_loc(&tf.translation);
        //let neighbours = map.get_nearby_ent(&map_coord);

        //let val = measure_coherence(ent, &behaviours, neighbours, rules.perception_range);
    }
}

pub fn debug_separation(
    query: Query<(Entity, &Transform, &Separation, &DebugBoid)>,
    // behaviours: Query<&Transform>,
    //  rules: Res<BoidsRules>,
    // mut lines: ResMut<DebugLines>,
    //map: Res<VoxelSpace>,
) {
    for (_ent, _tf, _, _) in &query {
        // Display only for debug_cohesion enabled behaviours

        //let map_coord = map.global_to_map_loc(&tf.translation);
        //let neighbours = map.get_nearby_ent(&map_coord);

        //let val = measure_separation(ent, &behaviours, neighbours, rules.perception_range);

        //    lines.line_colored(tf.translation, tf.translation + val, 0.0, Color::ANTIQUE_WHITE);
    }
}

fn debug_alignment(
    query: Query<(Entity, &Transform, &Velocity, &Alignment, &DebugBoid)>,
    //list: Query<(&Transform, &Velocity)>,
    //rules: Res<BoidsRules>,
    //map: Res<VoxelSpace>,
    // mut lines: ResMut<DebugLines>,
) {
    for (_, _, _, _, debug_boid) in query.iter() {
        // Display only for debug_cohesion enabled behaviours
        if !debug_boid.show_alignment {
            continue;
        }

        //let map_coord = map.global_to_map_loc(&tf.translation);
        // let neighbours = map.get_nearby_ent(&map_coord);

        //let val = measure_alignment(ent, &list, neighbours, rules.perception_range);

        //   lines.line_colored(tf.translation, tf.translation + val, 0.0, Color::INDIGO);
    }
}

pub fn mouse_track(
    mut query: Query<(&mut Transform, &DebugBoid)>,
    wnds: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    debug: Res<DebugConfig>,
) {
    if !debug.track_mouse {
        return;
    }
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = wnds.single();

    /*
        RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };*/

    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        for (mut tf, debug) in query.iter_mut() {
            if !debug.track_mouse {
                continue;
            }
            tf.translation.x = world_pos.x - 5.0;
            tf.translation.y = world_pos.y + 5.0;
        }
    }
}

pub fn reset_color_system(mut query: Query<(&mut Sprite, &Boid), With<Boid>>) {
    for (mut sp, b) in &mut query {
        sp.color = b.color;
    }
}

pub fn debug_perception_range(
    query: Query<(Entity, &Transform, &Perception, &DebugBoid)>,
    mut list: Query<(&mut Sprite, &Boid), Without<DebugBoid>>,
) {
    for (_, _, per, _) in query.iter() {
        let nearby = &per.list;

        for &e in nearby {
            if let Ok((mut sp, _)) = list.get_mut(e) {
                sp.color = Color::ORANGE_RED;
            }
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct DebugColorTag(Color);

fn debug_tag_spatial_hash_system(
    mut commands: Commands,
    mut query: Query<(&DebugBoid, &Transform, &Perception)>,
) {
    for (_, _, per) in &mut query {
        let list = &per.list;
        for &ent in list {
            commands
                .entity(ent)
                .insert(DebugColorTag(Color::ORANGE_RED));
        }
    }
}
