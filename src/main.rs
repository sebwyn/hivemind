use bevy::prelude::*;
use hivemind::terrain::*;
use bevy_asset_loader::prelude::*;
use hivemind::GameState;
use hivemind::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_plugins(TerrainGenerator)
        .add_plugins(PlayerPlugin)
        .run();
}
