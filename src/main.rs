use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};
use parascape::{player, obstacle};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
            )
        .add_systems(Startup, (setup_game, player::setup))
        .add_systems(Update, (
            player::read_actions, 
            (
                player::calculate_x_movement,
                player::adjust_rotation
            ),
            (obstacle::add, obstacle::shift)
        ).chain())
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
