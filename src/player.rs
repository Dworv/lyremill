use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f64,
    pub offset: Vec3
}