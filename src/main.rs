use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, render::camera::ScalingMode, ecs::schedule::ScheduleLabel};

use parascape::GameState;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
            )
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands, assets: Res<AssetServer>) {
    let handle = assets.load("textures/buildings/windmill.png");
    commands.spawn(SpriteBundle {
        texture: handle,
        sprite: Sprite {
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        ..default()
    });
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(16.),
            ..default()
        },
        ..default()
    });
}
