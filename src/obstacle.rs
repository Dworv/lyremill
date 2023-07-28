use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_obstacles, move_obstacles));
    }
}

#[derive(Component)]
pub struct Obstacle;

fn add_obstacles(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    }
}

fn move_obstacles(mut query: Query<(&mut Obstacle, &mut Transform)>) {
    for (_, mut t) in &mut query {
        t.translation.x -= 10.;
    }
}