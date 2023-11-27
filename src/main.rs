use bevy::prelude::*;
use hivemind::terrain::*;
use bevy_asset_loader::prelude::*;
use hivemind::GameStates;
use hivemind::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::Loading).continue_to_state(GameStates::Playing),
        )
        // .add_plugins(TerrainGenerator)
        .add_plugins(PlayerPlugin)
        .run();
}
