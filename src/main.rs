use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};
use parascape::Player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
        )
        .add_systems(Startup, (setup_game, setup_player))
        .add_systems(Update, update_player)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("textures/plane.png"),
        transform: Transform {
            scale: Vec3::splat(5.),
            ..default()
        },
        ..default()
    }, Player { speed: 1., offset: Vec3::splat(0.)}));
}

fn update_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Player, &mut Transform)>) {
    let mut movement = 0.;
    if keyboard_input.pressed(KeyCode::W) {
        movement += 1.;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement -= 1.;
    }

    let (mut player, mut transform) = query.get_single_mut().unwrap();
    let distance = movement - player.offset.y;
    let y_adjustment = distance / 16.;
    player.offset.y += y_adjustment;
    transform.translation.y += y_adjustment * 100.;
}