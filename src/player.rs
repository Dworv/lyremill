use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DeltaMovement(0.))
            .insert_resource(PlayerActions::default())
            .add_systems(Startup, setup_player)
            .add_systems(Update, (read_player_actions, calculate_speed).chain());
    }
}

#[derive(Deref, Resource)]
pub struct DeltaMovement(f32);

#[derive(Default, Resource)]
pub struct PlayerActions{
    vert: f32,
    thrust: f32,
    gun: bool
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        },
    ));
}

fn read_player_actions(keyboard_input: Res<Input<KeyCode>>, mut actions: ResMut<PlayerActions>) {
    actions.thrust = 0.;
    if keyboard_input.pressed(KeyCode::A) {
        actions.thrust -= 1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        actions.thrust += 1.;
    }
    actions.vert = 0.;
    if keyboard_input.pressed(KeyCode::W) {
        actions.vert -= 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        actions.vert += 1.;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        actions.gun = true;
    }
}

const ACCEL: f32 = 1.;
const MIN_SPEED: f32 = 100.;
const MAX_SPEED: f32 = 1000.;

fn calculate_speed(
    player_actions: Res<PlayerActions>,
    time: Res<Time>,
    mut movement: ResMut<DeltaMovement>,
    mut query: Query<(&mut Player)>,
) {
    let (mut player) = query.get_single_mut().unwrap();
    player.speed += (player_actions.thrust * ACCEL * time.delta_seconds()).min(0.).max(1.);
    movement.0 = MIN_SPEED + player.speed * (MAX_SPEED - MIN_SPEED);
}
