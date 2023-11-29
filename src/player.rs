use std::time::Duration;

use bevy_ecs_ldtk::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::{loading_state::LoadingStateAppExt, asset_collection::AssetCollection};
use bevy_rapier2d::{geometry::{Collider, Friction, ColliderMassProperties}, dynamics::{RigidBody, Velocity, LockedAxes, GravityScale, CoefficientCombineRule}};

use crate::GameState;

const WALK_FRAME_DURATION: Duration = Duration::from_millis(100);

#[derive(Component, Default)]
pub struct Player;

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle("player/48x48/idle.png", 48., 48., 10, 1, 0., 0., 0)]
    sprite: SpriteSheetBundle,
    player: Player,
}

#[derive(Default, Component)]
struct Animation {
    current_frame: usize,
    timer: Timer,
    frame_count: usize
}

impl Animation {
    fn new(frame_count: usize, frame_time: Duration) -> Self {
        Animation {
            timer: Timer::new(frame_time, TimerMode::Repeating),
            current_frame: 0,
            frame_count
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_collection_to_loading_state::<_, PlayerAssets>(GameState::Loading)
            .add_systems(OnEnter(GameState::Playing), spawn_camera)
            .add_systems(Update, (do_walk_animation, spawn_player).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    _player_texture: Res<PlayerAssets>,
    mut player: Query<Entity, Added<Player>> 
) {
    for entity in player.iter_mut() {
        println!("found entity!");
        commands.entity(entity).insert((
            ColliderBundle {
                collider: Collider::cuboid(8., 16.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..Default::default()
            },
            Animation::new(10, WALK_FRAME_DURATION)
        ));
    }
}

// fn control_player(
//     keys: Res<Keys
//     mut player: Query<&mut Transform, With<Player>> 
// ) {

// }

fn do_walk_animation(
    time: Res<Time>,
    mut player_query: Query<(&mut TextureAtlasSprite, &mut Animation), With<Player>>
) {
    let Ok((mut sprite, mut animation)) = player_query.get_single_mut() else {
        return
    };

    if animation.timer.finished() {
        let next_frame = (animation.current_frame + 1) % animation.frame_count;
        sprite.index = next_frame;

        animation.current_frame = next_frame;
    }

    animation.timer.tick(time.delta());
}


#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas(tile_size_x = 80., tile_size_y = 64., columns = 16, rows = 1))]
    #[asset(path = "player/80x64/katana_attack_sheathe.png")]
    walk: Handle<TextureAtlas>
}