use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_prototype_debug_lines::DebugLines;
use flock_sim::boids::{Boid, BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, GameRules, measure_alignment, measure_coherence, measure_separation, WorldBoundForce};
use flock_sim::physics::{Spatial, Velocity};


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

#[derive(Resource, Inspectable, Default)]
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
        static DEBUG: &str = "debug";

        app.add_stage_after(CoreStage::PostUpdate, DEBUG, SystemStage::parallel());
        app.add_system_to_stage(DEBUG, debug_separation);
        app.add_system_to_stage(DEBUG, debug_cohesion);
        app.add_system_to_stage(DEBUG, debug_alignment);
        app.add_system_to_stage(DEBUG, debug_perception_range);
        app.add_system_to_stage(DEBUG, debug_world_bounds);
        app.add_system_to_stage(DEBUG, mouse_track);
        app.add_system_to_stage(DEBUG, color_debug_boid_system);
        app.add_system_to_stage(DEBUG, debug_tag_spatial_hash_system);
        app.add_system_to_stage(DEBUG, debug_color_spatial_hash_system);

        app.add_plugin(InspectorPlugin::<DebugConfig>::new());
        app.insert_resource(DebugConfig {
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
    query: Query<(&Transform, &WorldBoundForce, &DebugBoid)>,
    mut lines: ResMut<DebugLines>,
    rules: Res<GameRules>,
    config: Res<DebugConfig>,
) {
    if !config.display_bound {
        return;
    }

    let start = vec3(rules.left, rules.top, 0.0);
    let end = vec3(rules.right, rules.top, 0.0);
    let duration = 0.0;     // Duration of 0 will show the line for 1 frame.
    lines.line_colored(start, end, duration, Color::BLACK);

    let start = vec3(rules.left, rules.bottom, 0.0);
    let end = vec3(rules.right, rules.bottom, 0.0);
    let duration = 0.0;     // Duration of 0 will show the line for 1 frame.
    lines.line_colored(start, end, duration, Color::BLACK);

    for (tf, bound, _) in query.iter(){
        lines.line_colored(tf.translation, tf.translation + bound.force, duration, Color::CYAN);
    }
}

pub fn debug_cohesion(
    query: Query<(Entity, &Transform, &BoidsCoherence, &DebugBoid)>,
    boids: Query<&Transform>,
    rules: Res<BoidsRules>,
    mut lines: ResMut<DebugLines>,
    map: Res<Spatial>,
) {
    for (ent, tf, _, debug) in query.iter() {
        // Display only for debug_cohesion enabled boids
        if !debug.show_cohesion {
            continue;
        }

        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        let val = measure_coherence(ent, &boids, neighbours, rules.perception_range);

        lines.line_colored(tf.translation, tf.translation + val, 0.0, Color::GREEN);
    }
}

pub fn debug_separation(
    query: Query<(Entity, &Transform, &BoidsSeparation, &DebugBoid)>,
    boids: Query<&Transform>,
    rules: Res<BoidsRules>,
    mut lines: ResMut<DebugLines>,
    map: Res<Spatial>,
) {
    for (ent, tf, _, _) in &query {
        // Display only for debug_cohesion enabled boids

        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        let val = measure_separation(ent, &boids, neighbours, rules.perception_range);

        lines.line_colored(tf.translation, tf.translation + val, 0.0, Color::ANTIQUE_WHITE);
    }
}

fn debug_alignment(
    query: Query<(Entity, &Transform, &Velocity, &BoidsAlignment, &DebugBoid)>,
    list: Query<(&Transform, &Velocity)>,
    rules: Res<BoidsRules>,
    map: Res<Spatial>,
    mut lines: ResMut<DebugLines>,
) {
    for (ent, tf, _, _, debug_boid) in query.iter() {
        // Display only for debug_cohesion enabled boids
        if !debug_boid.show_alignment {
            continue;
        }

        let map_coord = map.global_to_map_loc(&tf.translation, rules.perception_range);
        let neighbours = map.get_nearby_ent(&map_coord);

        let val = measure_alignment(ent, &list, neighbours, rules.perception_range);

        lines.line_colored(tf.translation, tf.translation + val, 0.0, Color::INDIGO);
    }
}

pub fn mouse_track(
    mut query: Query<(&mut Transform, &DebugBoid)>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    wnds: Res<Windows>,
    debug: Res<DebugConfig>,
) {
    if !debug.track_mouse {
        return;
    }
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

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

pub fn debug_perception_range(
    query: Query<(Entity, &Transform, &DebugBoid)>,
    mut list: Query<(Entity, &Transform, &mut Sprite, &Boid), Without<DebugBoid>>,
    config: Res<DebugConfig>,
    rules: Res<BoidsRules>,
) {
    if !config.display_perceived { return; }
    for (ent, tf, debug) in query.iter() {
        if !debug.show_perception_range {
            continue;
        }

        for (other_ent, other_tf, mut sprite, _) in &mut list {
            if ent == other_ent { continue; }

            sprite.color = Color::BLUE;

            let distance = other_tf.translation.distance(tf.translation);
            if distance < rules.perception_range {
                sprite.color = Color::PURPLE;
            }
        }
    }


}

fn color_debug_boid_system(
    mut query: Query<(&DebugBoid, &mut Sprite)>,
) {
    for (debug, mut sprite) in &mut query {
        sprite.color = debug.color;
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct SpatialColorDebug(Color);

fn debug_tag_spatial_hash_system(
    mut commands: Commands,
    mut query: Query<(&DebugBoid, &Transform, &mut Sprite)>,
    boid: Res<BoidsRules>,
    hash: ResMut<Spatial>,
) {
    for (debug, tf, mut sprite) in &mut query {
        if !debug.spatial_hash { continue; }

        sprite.color = debug.color;
        let map_pos = hash.global_to_map_loc(&tf.translation, boid.perception_range);
        let values = hash.get_nearby_ent(&map_pos);

        for ent in values {
            commands.entity(ent).insert(SpatialColorDebug(Color::ORANGE_RED));
        }
    }
}

fn debug_color_spatial_hash_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, Option<&SpatialColorDebug>), With<Boid>>,
) {
    for (ent, mut sp, debug) in &mut query {
        if let Some(dbg) = debug {
            sp.color = dbg.0;
            commands.entity(ent).remove::<SpatialColorDebug>();
        } else {
            sp.color = Color::BLUE;
        }
    }
}