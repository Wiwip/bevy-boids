use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Boid;


#[derive(Component, Default)]
pub struct Perception{
    pub range: f32,
}