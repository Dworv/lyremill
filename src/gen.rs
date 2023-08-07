use bevy::prelude::*;

use tilemap::prelude::*;

#[derive(Component)]
pub struct Grass;

pub fn grass(
    mut commands: Commands,
    query: Query<Entity, With<Grass>>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
    let handle = assets.load("textures/grounds/grass.png");
    for x in -7..=8 {
        for y in -7..=8 {
            commands.spawn((
                SpriteBundle {
                    texture: handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1., 1.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x as f32 - 0.5, y as f32 - 0.5, 0.),
                    ..default()
                },
                Grass,
            ));
        }
    }
}
