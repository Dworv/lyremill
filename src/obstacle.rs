use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::player::DeltaMovement;

#[derive(Component)]
pub struct Obstacle;

pub fn add(mut commands: Commands, asset_server: Res<AssetServer>) {
    if thread_rng().gen_ratio(1, 100) {
        commands.spawn((
            Obstacle,
            SpriteBundle {
                texture: asset_server.load("textures/meteor.png"),
                transform: Transform {
                    translation: Vec3 { x: 10., y: thread_rng().gen_range(-100.0..100.0), z: 0. },
                    ..default()
                },
                ..default()
            }
        ));
        println!("added")
    }
}

pub fn shift(mut query: Query<(&mut Obstacle, &mut Transform)>, movement: Res<DeltaMovement>, time: Res<Time>) {
    for (_, mut t) in &mut query {
        t.translation -= **movement * time.delta_seconds();
    }
}