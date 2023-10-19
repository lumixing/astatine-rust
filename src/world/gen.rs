use bevy::{prelude::*, math::ivec2};

use crate::states::GameState;

use super::storage::WorldStorage;

pub fn generate(mut commands: Commands) {
    let mut world = WorldStorage::new();

    world.set_block(ivec2(1, 1), 1);
    world.set_block(ivec2(2, 2), 2);
    world.set_block(ivec2(32, 32), 3);
    world.set_block(ivec2(16, 48), 4);

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
    info!("generated! moving to ingame state");
}