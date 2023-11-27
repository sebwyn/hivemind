use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::{loading_state::LoadingStateAppExt, asset_collection::AssetCollection};

use crate::GameStates;

const WALK_FRAME_DURATION: Duration = Duration::from_millis(100);


#[derive(Component)]
struct Player;

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
            .add_collection_to_loading_state::<_, PlayerAssets>(GameStates::Loading)
            .add_systems(OnEnter(GameStates::Playing), spawn_player)
            .add_systems(Update, do_walk_animation.run_if(in_state(GameStates::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    player_texture: Res<PlayerAssets>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: player_texture.walk.clone(),
            transform: Transform::from_scale(Vec3::splat(8.)),
            ..Default::default()
        },
        Player,
        Animation::new(16, WALK_FRAME_DURATION)
    ));
}

fn do_walk_animation(
    time: Res<Time>,
    mut player_query: Query<(&mut TextureAtlasSprite, &mut Animation), With<Player>>
) {
    let (mut sprite, mut animation) = player_query.single_mut();

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