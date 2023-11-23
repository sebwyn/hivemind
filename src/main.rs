use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::Anchor,
    utils::HashMap, audio::Decodable, transform::commands,
};

#[derive(Component, PartialEq, Clone, Copy)]
enum Team {
    FRIEND,
    FOE
}

#[derive(Component)]
struct Character {
    move_speed: f32,
}

#[derive(Component)]
struct HurtBox {
    damage: f32
}

#[derive(Component)]
struct HitBox;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct Health {
    health: f32,
    max_health: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Focus {
    position: Vec2,
}

#[derive(Component)]
struct Fuse(Timer);

#[derive(Component)]
struct Attacking;

#[derive(Component)]
struct Hornet;

#[derive(Resource, Default)]
struct GameAssets(HashMap<String, Handle<TextureAtlas>>);

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .insert_resource(GameAssets::default())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, create_world))
        .add_systems(Update, move_player)
        .add_systems(Update, camera_follows_player)
        .add_systems(Update, update_health_bars)
        .add_systems(Update, do_hornet_attack)
        .add_systems(Update, do_physics)
        .add_systems(Update, check_collisions)
        .add_systems(Update, despawn_dead_characters)
        .run();
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

fn spawn_character(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) -> Entity {
    let character = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 1,
                custom_size: Some(Vec2::splat(24.)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .id();

    let health_bar_background = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(30.0, 5.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.)),
            ..default()
        })
        .id();

    let health_bar_full = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(30.0, 5.0)),
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-15.0, 15.0, 0.)),
                ..default()
            },
            HealthBar,
        ))
        .id();

    let hit_box = commands.spawn((
        HitBox,
        TransformBundle::from_transform(Transform::from_scale(Vec3::splat(0.7*24.)))
    )).id();

    commands
        .entity(character)
        .push_children(&[health_bar_background, health_bar_full, hit_box]);

    character
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("robot_pack/Robots/Hornet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let hornet = spawn_character(&mut commands, texture_atlas_handle);
    commands.entity(hornet).insert((
        Player,
        Health {
            max_health: 100.,
            health: 100.,
        },
        Character { move_speed: 5. },
        Hornet,
        Team::FRIEND,
    ));

    let texture_handle = asset_server.load("robot_pack/Soldiers/Grenadier-Class.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 7, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let dummy = spawn_character(&mut commands, texture_atlas_handle);
    commands.entity(dummy).insert((Health {
        max_health: 100.,
        health: 75.,
    }, Team::FOE));

    let texture_handle = asset_server.load("robot_pack/Projectiles/bullets_plasma.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    game_assets
        .0
        .insert("hornet_projectile".to_string(), texture_atlas_handle);
}

fn move_player(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Transform, &Character), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (entity, mut transform, character) = player_query.single_mut();

    if keys.pressed(KeyCode::W) {
        transform.translation.y += character.move_speed;
    }

    if keys.pressed(KeyCode::A) {
        transform.translation.x -= character.move_speed;
    }

    if keys.pressed(KeyCode::S) {
        transform.translation.y -= character.move_speed;
    }

    if keys.pressed(KeyCode::D) {
        transform.translation.x += character.move_speed;
    }

    if keys.just_pressed(KeyCode::Space) {
        let (camera, camera_transform) = camera_query.single();

        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };

        commands
            .entity(entity)
            .insert((Focus { position: point }, Attacking));
    }
}

fn camera_follows_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn update_health_bars(
    mut health_bar: Query<(&Parent, &mut Transform), With<HealthBar>>,
    character_health: Query<&Health>,
) {
    for (character, mut transform) in health_bar.iter_mut() {
        let health = character_health.get(character.get()).unwrap();
        transform.scale.x = health.health / health.max_health;
    }
}

fn do_hornet_attack(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    hornet_query: Query<(Entity, &Focus, &Transform, &Team), (With<Hornet>, With<Attacking>)>,
) {
    for (e, attacking, transform, team) in hornet_query.iter() {
        let direction = (attacking.position - transform.translation.xy()).normalize();
        const SPEED: f32 = 15.;

        let stinger = commands.spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets.0.get("hornet_projectile").unwrap().clone(),
                sprite: TextureAtlasSprite{
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
            *team
        )).id();

        let stinger_hurt_box = commands.spawn(( 
            HurtBox {damage: 10.},
            TransformBundle::from_transform(
                Transform::from_scale(Vec3::splat(7.))
            )
        )).id();

        commands.entity(stinger).push_children(&[stinger_hurt_box]);
        commands.entity(e).remove::<Attacking>();
    }
}

fn do_physics(
    mut commands: Commands,
    mut moving_objects: Query<(&mut Transform, &Velocity, &mut Fuse, Entity)>,
    time: Res<Time>
) {
    for (mut transform, velocity, mut fuse, entity) in moving_objects.iter_mut() {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
        fuse.0.tick(time.delta());

        if fuse.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    hit_boxes: Query<(&Parent, &GlobalTransform), With<HitBox>>,
    hurt_boxes: Query<(&Parent, &GlobalTransform, &HurtBox)>,
    mut hit_parent_q: Query<(&mut Health, &Team)>,
    hurt_parent_q: Query<&Team>
) {

    let mut the_notebook_from_the_deathnote = Vec::new();
    //let hurt_boxes_list = hurt_boxes.iter().collect();
    for (hit_p, hit_transform) in hit_boxes.iter() {
        let x1 = hit_transform.translation().x - hit_transform.compute_transform().scale.x/2.;
        let x2 = hit_transform.translation().x + hit_transform.compute_transform().scale.x/2.;

        let y1 = hit_transform.translation().y - hit_transform.compute_transform().scale.y/2.;
        let y2 = hit_transform.translation().y + hit_transform.compute_transform().scale.y/2.;
        for (hurt_p, hurt_transform, HurtBox{damage}) in hurt_boxes.iter() {
            let x1_hurt = hurt_transform.translation().x - hurt_transform.compute_transform().scale.x/2.;
            let x2_hurt = hurt_transform.translation().x + hurt_transform.compute_transform().scale.x/2.;

            let y1_hurt = hurt_transform.translation().y - hurt_transform.compute_transform().scale.y/2.;
            let y2_hurt = hurt_transform.translation().y + hurt_transform.compute_transform().scale.y/2.;

            if (x2_hurt > x1 && x1_hurt < x2) && (y2_hurt > y1 && y1_hurt < y2) {
                let (mut hit_health, hit_team) = hit_parent_q.get_mut(hit_p.get()).unwrap();
                let hurt_team = hurt_parent_q.get(hurt_p.get()).unwrap();
                if hurt_team != hit_team {
                    hit_health.health -= damage;
                    the_notebook_from_the_deathnote.push(hurt_p.get());
                }
            }
        }
    }

    the_notebook_from_the_deathnote.dedup();
    for victim in the_notebook_from_the_deathnote.into_iter() {
        commands.entity(victim).despawn_recursive();
    }
}

fn despawn_dead_characters(
    mut commands: Commands,
    characters: Query<(Entity,&Health)>,
) {
    for (entity, health) in characters.iter() {
        if health.health <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}