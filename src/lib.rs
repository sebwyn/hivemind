use bevy::ecs::schedule::States;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, States)]
pub enum GameStates {
    #[default]
    Loading,
    Playing,
}

pub mod terrain;
pub mod player;
pub use player::*;