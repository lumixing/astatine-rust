use bevy::prelude::*;

use crate::states::GameState;

pub mod boxy;
pub mod item;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<item::SpawnItem>();

        // app.add_systems(OnEnter(GameState::InGame), (
        //     boxy::spawn,
        // ).chain());

        app.add_systems(Update, (
            item::spawn
        ).run_if(in_state(GameState::InGame)).chain());
    }
}