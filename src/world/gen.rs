use bevy::prelude::*;

use crate::states::GameState;

use super::storage::WorldStorage;

pub fn generate(mut commands: Commands) {
    let world = WorldStorage::new();

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
    info!("generated! moving to ingame state");
}