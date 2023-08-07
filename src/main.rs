use std::time::Duration;

use bevy::{
    asset::ChangeWatcher, prelude::*, render::camera::ScalingMode, window::WindowResolution,
};

use lyremill::{gen, GameState};

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
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1000., 1000.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Generating), gen::grass)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(16.),
            ..default()
        },
        ..default()
    });
}
