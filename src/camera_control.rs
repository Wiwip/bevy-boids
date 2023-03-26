use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::Input;
use bevy::prelude::{
    Camera, EventReader, MouseButton, OrthographicProjection, Query, Res, Transform,
};
use bevy_rapier2d::na::clamp;

pub fn camera_drag(
    mut query: Query<(&Camera, &mut Transform, &OrthographicProjection)>,
    input: Res<Input<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
) {
    let (_, mut tf, proj) = query.single_mut();
    if input.pressed(MouseButton::Middle) {
        for ev in motion.iter() {
            tf.translation.x -= ev.delta.x * proj.scale;
            tf.translation.y += ev.delta.y * proj.scale;
        }
    }
}

pub fn camera_zoom(
    mut query: Query<(&Camera, &mut OrthographicProjection)>,
    mut scroll: EventReader<MouseWheel>,
) {
    let (_, mut proj) = query.single_mut();
    for ev in scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                proj.scale -= ev.y / 10.;
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
            MouseScrollUnit::Pixel => {
                proj.scale -= ev.y / 10.;
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
        }
        proj.scale = clamp(proj.scale, 0.25, 1.25);
    }
}
