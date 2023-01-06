use bevy::ecs::query::QuerySingleError;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_debug_lines::DebugLines;
use rand_distr::num_traits::pow;
use crate::boids::{BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, DesiredVelocity, GameRules, Movement, Particle, WorldBound};
use crate::helper::velocity_angle;

#[derive(Component)]
pub struct Dbg;

#[derive(Component)]
pub struct DebugSeparation;

#[derive(Component)]
pub struct MouseTracking;

#[derive(Resource, Inspectable, Default)]
pub struct DebugConfig {
    pub track_mouse: bool,
    pub freeze_world: bool,
    pub display_separation_sum: bool,
    pub display_separation: bool,
    pub display_cohesion: bool,
    pub display_alignment: bool,
    pub current_velocity: bool,
    pub desired_velocity: bool,
    pub display_bound: bool,
    pub debug_location: Vec3,
    pub debug_vector_mag: f32,
}

pub fn debug_print(mut query: Query<(&Transform, &Movement, &BoidsCoherence, &BoidsAlignment, &BoidsSeparation), With<Dbg>>) {
    let Ok((tf, mov, cohe, ali, sep)) = query.get_single()
    else {
        return;
    };
    //println!("V:{} P:{} Coh:{} Ali:{} Sep:{}", mov.vel.length(), tf.translation, cohe.coherence, ali.alignment, sep.separation);
}


pub fn forces_debug(
    query: Query<(&Transform, &Movement, &Dbg, &BoidsAlignment, &BoidsCoherence, &BoidsSeparation, &WorldBound, &DesiredVelocity)>,
    config: Res<DebugConfig>,
    mut lines: ResMut<DebugLines>
) {
    let Ok((tf, mov, debug, ali, coh, sep, bound, des)) = query.get_single()
    else { return; };

    let duration = 0.0;     // Duration of 0 will show the line for 1 frame.
    let acc = bound.force + ali.force + coh.force + sep.force + des.force;
    if config.display_bound {
        lines.line_colored(tf.translation, tf.translation + bound.force, duration, Color::CYAN);
    }

    if config.display_alignment{
        lines.line_colored(tf.translation, tf.translation + ali.force, duration, Color::PURPLE);
        lines.line_colored(config.debug_location, config.debug_location + ali.force * config.debug_vector_mag, duration, Color::PURPLE);
    }

    if config.display_cohesion{
        lines.line_colored(tf.translation, tf.translation + coh.force, duration, Color::GREEN);
        lines.line_colored(config.debug_location, config.debug_location + coh.force * config.debug_vector_mag, duration, Color::GREEN);
    }

    if config.display_separation_sum {
        lines.line_colored(tf.translation, tf.translation + sep.force, duration, Color::RED);
        lines.line_colored(config.debug_location, config.debug_location + sep.force * config.debug_vector_mag, duration, Color::RED);
    }

    if config.desired_velocity {
        lines.line_colored(tf.translation, tf.translation + des.force, duration, Color::WHITE);
    }

    if config.current_velocity {
        lines.line_colored(tf.translation, tf.translation + mov.vel, duration, Color::PURPLE);

        lines.line_colored(tf.translation, tf.translation + acc, duration, Color::BLACK);
        lines.line_colored(config.debug_location, config.debug_location + acc * config.debug_vector_mag, duration, Color::ORANGE);
    }
}

pub fn debug_lines(mut lines: ResMut<DebugLines>, rules: Res<GameRules>) {
    let start = vec3(rules.left, rules.top, 0.0);
    let end = vec3(rules.right, rules.top, 0.0);
    let duration = 0.0;     // Duration of 0 will show the line for 1 frame.
    lines.line_colored(start, end, duration, Color::BLACK);

    let start = vec3(rules.left, rules.bottom, 0.0);
    let end = vec3(rules.right, rules.bottom, 0.0);
    let duration = 0.0;     // Duration of 0 will show the line for 1 frame.
    lines.line_colored(start, end, duration, Color::BLACK);
}

pub fn debug_cohesion(
    query: Query<(Entity, &Transform, &Movement, &BoidsCoherence), With<Dbg>>,
    mut list: Query<(Entity, &Transform, &mut Sprite), With<Particle>>,
    rules: Res<BoidsRules>,
    mut lines: ResMut<DebugLines>
){
    let (ent, tf, mov, coh) = query.single();
    let mut vec = vec3(0.0, 0.0, 0.0);
    let mut count = 0;

    for (other_ent, other_tf, mut sprite) in &mut list {
        if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

        sprite.color = Color::BLUE;

        let distance = other_tf.translation.distance(tf.translation);
        if distance < rules.perception_range {
            vec += other_tf.translation;
            count += 1;
            sprite.color = Color::PURPLE;
        }
    }

    match count {
        0 => {  }, // no division by zero
        _ => {
            let mut steering = vec / count as f32;
            steering = steering - tf.translation;
            lines.line_colored(tf.translation, tf.translation + steering, 0.0, Color::BLACK); // * res.coherence_factor;
        },
    }
}

pub fn debug_separation(
    query: Query<(Entity, &Transform), With<DebugSeparation>>,
    mut list: Query<(Entity, &Transform, &mut Sprite), With<Particle>>,
    rules: Res<BoidsRules>,
    debug_config: Res<DebugConfig>,
    mut lines: ResMut<DebugLines>
){
    if !debug_config.display_separation { return; }
    let Ok((ent, tf)) = query.get_single()
        else { println!("No debug particle for separation system"); return; };
    let mut vec = vec3(0.0, 0.0, 0.0);
    let mut count = 0;

    for (other_ent, other_tf, mut sprite) in &mut list {
        if ent == other_ent { continue; } // Don't count current entity as part of the center of flock

        let distance = other_tf.translation.distance_squared(tf.translation);
        if distance <= pow(rules.desired_separation, 2) {
            let diff = other_tf.translation - tf.translation;
            let unit_diff = diff / diff.length();
            let pressure = unit_diff * (rules.desired_separation / diff.length());

            vec = vec - diff;
            count += 1;
            lines.line_colored(tf.translation, tf.translation - pressure, 0.0, Color::ORANGE);
        }
    }
}

pub fn mouse_track(
    mut query: Query<(&mut Transform), With<MouseTracking>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    wnds: Res<Windows>,
    debug: Res<DebugConfig>,
) {
    if !debug.track_mouse { return; }
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

        match query.get_single_mut()  {
            Ok(mut tf) => {
                tf.translation.x = world_pos.x - 5.0;
                tf.translation.y = world_pos.y + 5.0;
            }
            Err(_) => {}
        }
    }

}