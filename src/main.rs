mod character;
mod dummy;
mod health_bar;
mod hitboxes;
mod hornet;
mod player;
mod projectile;
mod world;

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct SpriteSheetHandles(HashMap<String, Handle<TextureAtlas>>);

fn main() {
    App::new()
        .insert_resource(SpriteSheetHandles::default())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            world::WorldPlugin,
            health_bar::HealthBarPlugin,
            character::CharacterPlugin,
            dummy::DummyPlugin,
            hitboxes::HitboxPlugin,
            hornet::HornetPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut handles: ResMut<SpriteSheetHandles>,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    dummy::spawn_dummy(&mut commands, &mut handles, &asset_server, &mut atlases);
    let player = hornet::spawn_hornet(&mut commands, &mut handles, &asset_server, &mut atlases);

    commands.entity(player).insert(player::Player);
}

fn get_spritesheet_handle_with_cache(
    path: String,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    handles: &mut ResMut<SpriteSheetHandles>,
    asset_server: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let handle;
    if let Some(h) = handles.0.get(&path) {
        handle = h.clone()
    } else {
        let texture_handle = asset_server.load(path);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, tile_size, columns, rows, None, None);
        let h = &atlases.add(texture_atlas);
        handles
            .0
            .insert("hornet_stinger_sprite".to_string(), h.clone());
        handle = h.clone();
    }

    handle
}
