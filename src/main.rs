use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(AssetPlugin { watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)), ..default()}))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/plane.png"),
        transform: Transform {
            scale: Vec3::splat(5.),
            ..default()
        },
        ..default()
    });
}