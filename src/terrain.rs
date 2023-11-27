use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection, LdtkWorldBundle};

struct LevelLdtkBundle;

use crate::GameStates;

#[derive(Default)]
pub struct TerrainGenerator;

impl Plugin for TerrainGenerator {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_systems(Startup, spawn_level);
    }
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Running!");
    
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("TestPlatforms.ldtk"),
        ..Default::default()
    });
}