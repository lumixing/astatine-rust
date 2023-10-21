use bevy::prelude::*;

use crate::states::GameState;

pub mod boxy;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (
            boxy::spawn,
        ).chain());

        // app.add_systems(Update, (
        //     player::movement,
        //     player::update_positions,
        //     player::follow_player,
        // ).run_if(in_state(GameState::InGame)).chain());
    }
}