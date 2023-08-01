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
    pub rot_speed: f32
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
            speed: 0.,
            rot_speed: 0.
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
        actions.thrust.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        actions.thrust.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        actions.gun = true;
    }
}

pub const ACCEL: f32 = 1.;
pub const MIN_MOVE_SPEED: f32 = 100.;
pub const MAX_MOVE_SPEED: f32 = 1000.;

pub fn calculate_x_movement(
    player_actions: Res<Actions>,
    time: Res<Time>,
    mut movement: ResMut<DeltaMovement>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.get_single_mut().unwrap();
    let speed_change = player_actions.thrust.x * ACCEL * time.delta_seconds();
    player.speed = (player.speed + speed_change).min(1.).max(0.);
    movement.x = MIN_MOVE_SPEED + player.speed * (MAX_MOVE_SPEED - MIN_MOVE_SPEED);
}

pub const ROT_ACCEL: f32 = 1.;

pub fn adjust_rotation(
    player_actions: Res<Actions>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.get_single_mut().unwrap();
    let rot_speed_change = player_actions.thrust.y * ROT_ACCEL * time.delta_seconds();
    player.rot_speed = (player.rot_speed + rot_speed_change).min(1.).max(-1.);
    transform.rotate_z(player.rot_speed * time.delta_seconds());
}

