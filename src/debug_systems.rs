use bevy::ecs::system::Command;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_prototype_debug_lines::DebugLines;
use rand_distr::num_traits::{pow, Pow};
use crate::boids::{BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, DesiredVelocity, GameRules, Movement, Boid, WorldBoundForce, alignment_system};
use crate::physics::Spatial;


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
    pub freeze_world: bool,
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

#[derive(Resource, Inspectable, Default)]
pub struct DebugSystemsStatus {
    pub separation_system: bool,
    pub cohesion_system: bool,
    pub alignment_system: bool,
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
        app.add_system_to_stage(DEBUG, other_display_debug);
        app.add_system_to_stage(DEBUG, mouse_track);
        app.add_system_to_stage(DEBUG, color_debug_boid_system);
        app.add_system_to_stage(DEBUG, debug_tag_spatial_hash_system);
        app.add_system_to_stage(DEBUG, debug_color_spatial_hash_system);

        app.add_plugin(InspectorPlugin::<DebugConfig>::new());
        app.insert_resource(DebugConfig {
            debug_location: vec3(-500.0, 400.0, 0.0),
            debug_vector_mag: 10.0,
            display_separation_sum: true,
            display_separation: true,
            display_cohesion: true,
            display_alignment: true,
            display_perceived: true,
            ..default()
        });

        app.add_plugin(InspectorPlugin::<DebugSystemsStatus>::new());
        app.insert_resource(DebugSystemsStatus::default());
    }
}

pub fn other_display_debug(
    query: Query<(&Transform, &Movement, &BoidsAlignment, &BoidsCoherence, &BoidsSeparation, &WorldBoundForce, &DesiredVelocity, &DebugBoid)>,
    config: Res<DebugConfig>,
    mut lines: ResMut<DebugLines>,
) {
    for (tf, mov, ali, coh, sep, bound, des, boid) in query.iter(){
        let duration = 0.0;     // Duration of 0 will show the line for 1 frame.
        let acc = bound.force + ali.force + coh.force + sep.force + des.force;

        // Don't display all other vectors // TODO better handling of separate vector viewer
        if !boid.track_mouse {
            continue;
        }
        if config.display_alignment {
            lines.line_colored(config.debug_location, config.debug_location + ali.force * config.debug_vector_mag, duration, Color::PURPLE);
        }

        if config.display_cohesion {
            lines.line_colored(config.debug_location, config.debug_location + coh.force * config.debug_vector_mag, duration, Color::GREEN);
        }

        if config.display_separation_sum {
            lines.line_colored(config.debug_location, config.debug_location + sep.force * config.debug_vector_mag, duration, Color::RED);
        }

        if config.velocity_info {
            lines.line_colored(tf.translation, tf.translation + mov.vel * config.debug_vector_mag, duration, Color::PURPLE);
            lines.line_colored(config.debug_location, config.debug_location + des.force * config.debug_vector_mag, duration, Color::WHITE);
            lines.line_colored(config.debug_location, config.debug_location + acc * config.debug_vector_mag, duration, Color::ORANGE);
        }
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

    for (tf, bound, debug) in query.iter(){
        lines.line_colored(tf.translation, tf.translation + bound.force, duration, Color::CYAN);
    }
}

pub fn debug_cohesion(
    query: Query<(Entity, &Transform, &BoidsCoherence, &DebugBoid)>,
    list: Query<(Entity, &Transform, &Boid)>,
    rules: Res<BoidsRules>,
    config: Res<DebugConfig>,
    mut status: ResMut<DebugSystemsStatus>,
    mut lines: ResMut<DebugLines>,
) {
    if !config.display_cohesion {
        status.cohesion_system = false;
        return;
    }

    for (ent, tf, coh, debug) in query.iter() {
        // Display only for debug_cohesion enabled boids
        if !debug.show_cohesion {
            continue;
        }

        let mut vec = vec3(0.0, 0.0, 0.0);
        let mut count = 0;


        for (other_ent, other_tf, boid) in &list {
            if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance(tf.translation);
            if distance < rules.perception_range {
                vec += other_tf.translation;
                count += 1;
            }
        }

        match count {
            0 => {} // No to division by zero.
            _ => {
                let mut steering = vec / count as f32;
                steering = steering - tf.translation;
                lines.line_colored(tf.translation, tf.translation + steering, 0.0, Color::GREEN);
            }
        }

        status.cohesion_system = true;
    }
}

pub fn debug_separation(
    query: Query<(Entity, &Transform, &BoidsSeparation, &DebugBoid)>,
    list: Query<(Entity, &Transform, &Boid)>,
    rules: Res<BoidsRules>,
    config: Res<DebugConfig>,
    mut status: ResMut<DebugSystemsStatus>,
    mut lines: ResMut<DebugLines>,
) {
    if !config.display_separation {
        status.separation_system = false;
        return;
    }

    for (ent, tf, sep, debug) in query.iter() {
        // Display only for debug_cohesion enabled boids
        if !debug.show_separation {
            continue;
        }

        let mut vec = vec3(0.0, 0.0, 0.0);
        let mut count = 0;

        for (other_ent, other_tf, boid) in &list {
            if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

            let distance = other_tf.translation.distance_squared(tf.translation);
            if distance <= pow(rules.desired_separation, 2) {
                let diff = other_tf.translation - tf.translation;
                let unit_diff = diff / diff.length();
                let pressure = unit_diff * (rules.desired_separation / diff.length());

                vec = vec - diff;
                count += 1;
                lines.line_colored(other_tf.translation, other_tf.translation + pressure, 0.0, Color::ORANGE);
            }
        }

        lines.line_colored(tf.translation, tf.translation + sep.force, 0.0, Color::ANTIQUE_WHITE);
        status.separation_system = true;
    }
}

fn debug_alignment(
    query: Query<(Entity, &Transform, &Movement, &BoidsAlignment, &DebugBoid)>,
    list: Query<(Entity, &Transform, &Movement, &Boid)>,
    rules: Res<BoidsRules>,
    config: Res<DebugConfig>,
    mut status: ResMut<DebugSystemsStatus>,
    mut lines: ResMut<DebugLines>,
) {
    if !config.display_alignment {
        status.alignment_system = false;
        return;
    }

    for (ent, tf, mov, ali, debug_boid) in query.iter() {
        // Display only for debug_cohesion enabled boids
        if !debug_boid.show_alignment {
            continue;
        }

        let mut vec = vec3(0.0, 0.0, 0.0);
        let mut count = 0;

        for (other_ent, other_tf, other_mov, boid) in &list {
            if ent == other_ent { continue; }

            let distance = other_tf.translation.distance_squared(tf.translation);
            if distance <= f32::pow(rules.perception_range, 2) {
                vec += other_mov.vel;
                count += 1;
            }
        }
        lines.line_colored(tf.translation, tf.translation + ali.force + 4.0, 0.0, Color::INDIGO);
        status.alignment_system = true;
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

        for (other_ent, other_tf, mut sprite, boid) in &mut list {
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
    mut lines: ResMut<DebugLines>,
) {
    for (debug, tf, mut sprite) in &mut query {
        if !debug.spatial_hash { continue; }

        sprite.color = debug.color;
        let map_pos = hash.global_to_map_loc(&tf.translation, boid.perception_range);
        let values = hash.get_nearby_transforms(&map_pos);

        for (ent, tf, mov) in values {
            commands.entity(ent).insert(SpatialColorDebug(Color::ORANGE_RED));
        }

        lines.line_colored(tf.translation, tf.translation + vec3(boid.perception_range, 0.0, 0.0), 0.0, Color::INDIGO);
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