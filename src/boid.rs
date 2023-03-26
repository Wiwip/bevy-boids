use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Boid {
    pub color: Color,
}


#[derive(Component, Default)]
pub struct Perception {
    pub range: f32,
}