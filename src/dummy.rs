use super::character::*;
use super::health_bar;
use super::hitboxes::*;
use bevy::prelude::*;

#[derive(Component)]
struct Dummy;

pub struct DummyPlugin;

impl Plugin for DummyPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn spawn_dummy(
    commands: &mut Commands,
    handles: &mut ResMut<super::SpriteSheetHandles>,
    asset_server: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let handle = super::get_spritesheet_handle_with_cache(
        "robot_pack/Soldiers/Grenadier-Class.png".to_string(),
        Vec2::splat(16.),
        5,
        7,
        handles,
        asset_server,
        atlases,
    );

    let dummy = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    custom_size: Some(Vec2::splat(24.)),
                    ..default()
                },
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            },
            Health {
                max_health: 100.,
                health: 75.,
            },
            Team::Foe,
            Dummy,
            Character { move_speed: 0. },
        ))
        .id();

    let health_bar = health_bar::spawn_health_bar(commands);

    let hit_box = commands
        .spawn((
            HitBox,
            TransformBundle::from_transform(Transform::from_scale(Vec3::splat(0.7 * 24.))),
        ))
        .id();

    commands.entity(dummy).push_children(&[health_bar, hit_box]);

    dummy
}
