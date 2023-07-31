use bevy::prelude::*;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct DeltaMovement(Vec3);

#[derive(Default, Resource)]
pub struct Actions{
    vert: f32,
    thrust: Vec3,
    gun: bool
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DeltaMovement::default());
    commands.insert_resource(Actions::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/plane.png"),
            transform: Transform {
                scale: Vec3::splat(5.),
                ..default()
            },
            ..default()
        },
        Player {
            speed: 0.
        },
    ));
}

pub fn read_actions(keyboard_input: Res<Input<KeyCode>>, mut actions: ResMut<Actions>) {
    actions.thrust = Vec3::splat(0.);
    if keyboard_input.pressed(KeyCode::A) {
        actions.thrust.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        actions.thrust.x += 1.;
    }
    actions.vert = 0.;
    if keyboard_input.pressed(KeyCode::W) {
        actions.thrust.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        actions.thrust.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        actions.gun = true;
    }
}

pub const ACCEL: f32 = 1.;
pub const MIN_SPEED: f32 = 100.;
pub const MAX_SPEED: f32 = 1000.;

pub fn calculate_speed(
    player_actions: Res<Actions>,
    time: Res<Time>,
    mut movement: ResMut<DeltaMovement>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.get_single_mut().unwrap();
    player.speed += (player_actions.thrust.x * ACCEL * time.delta_seconds()).min(1.).max(0.);
    movement.x = MIN_SPEED + player.speed * (MAX_SPEED - MIN_SPEED);
    println!("{}", movement.x);
}
