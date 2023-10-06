use bevy::prelude::States;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    WorldGeneration,
    InGame,
}