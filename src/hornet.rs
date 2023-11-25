use super::character::*;
use super::health_bar;
use super::hitboxes::*;
use super::projectile::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
struct Hornet;

pub struct HornetPlugin;
impl Plugin for HornetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, do_hornet_attack);
    }
}

pub fn spawn_hornet(
    commands: &mut Commands,
    handles: &mut ResMut<super::SpriteSheetHandles>,
    asset_server: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let handle = super::get_spritesheet_handle_with_cache(
        "robot_pack/Robots/Hornet.png".to_string(),
        Vec2::splat(24.),
        3,
        2,
        handles,
        asset_server,
        atlases,
    );

    let hornet = commands
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
                health: 100.,
            },
            Team::Friend,
            Hornet,
            Character { move_speed: 5. },
        ))
        .id();

    let health_bar = health_bar::spawn_health_bar(commands);

    let hit_box = commands
        .spawn((
            HitBox,
            TransformBundle::from_transform(Transform::from_scale(Vec3::splat(0.7 * 24.))),
        ))
        .id();

    commands
        .entity(hornet)
        .push_children(&[health_bar, hit_box]);

    hornet
}

fn do_hornet_attack(
    mut commands: Commands,
    mut handles: ResMut<super::SpriteSheetHandles>,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut hornet_query: Query<(Entity, &Focus, &Transform, &Team), (With<Hornet>, With<Attacking>)>,
) {
    let handle =
        super::get_spritesheet_handle_with_cache(
            "robot_pack/Projectiles/bullets_plasma.png".to_string(),
            Vec2::splat(16.),
            3,
            1,
            &mut handles,
            &asset_server,
            &mut atlases,
        );

    for (e, focus, transform, team ) in hornet_query.iter() {                
        let direction = (focus.position - transform.translation.xy()).normalize();
        const SPEED: f32 = 15.;

        let stinger = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: handle.clone(),
                    sprite: TextureAtlasSprite {
                        index: 1,
                        custom_size: Some(Vec2::splat(10.)),
                        ..default()
                    },
                    transform: Transform::from_scale(Vec3::splat(6.0))
                        .with_translation(transform.translation),
                    ..default()
                },
                Velocity(direction * SPEED),
                Fuse(Timer::new(Duration::from_secs(5), TimerMode::Once)),
                *team,
            ))
            .id();

        let stinger_hurt_box = commands
            .spawn((
                HurtBox { damage: 10. },
                TransformBundle::from_transform(Transform::from_scale(Vec3::splat(7.))),
            ))
            .id();

        commands.entity(stinger).push_children(&[stinger_hurt_box]);
        commands.entity(e).remove::<Attacking>();
    }
}
