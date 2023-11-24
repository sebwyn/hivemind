use bevy::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_world);
    }
}

fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("robot_pack/Tileset/tileset_arranged.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 2, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for x in 0..100 {
        for y in 0..100 {
            commands.spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(1),
                transform: Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(
                    x as f32 * 6.0 * 16.0,
                    y as f32 * 6.0 * 16.0,
                    -1.,
                )),
                ..default()
            },));
        }
    }
}
