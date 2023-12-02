use bevy::prelude::*;

use crate::states::GameState;

pub mod boxy;
pub mod item;
pub mod arrow;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<item::SpawnItem>();
        app.add_event::<arrow::SpawnArrow>();

        // app.add_systems(OnEnter(GameState::InGame), (
        //     boxy::spawn,
        // ).chain());

        app.add_systems(Update,
            (
                item::spawn,
                item::check_collisions,
                item::animate,
                arrow::spawn,
                arrow::rotate,
            ).run_if(in_state(GameState::InGame)).chain(),
        );
    }
}
