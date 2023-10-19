use bevy::prelude::*;

use crate::states::GameState;

pub mod player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (
            player::spawn_camera,
            player::spawn_player,
        ).chain());

        app.add_systems(Update, (
            player::movement,
            player::update_positions,
            player::follow_player,
        ).run_if(in_state(GameState::InGame)).chain());
    }
}