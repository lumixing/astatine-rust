use bevy::prelude::*;

use crate::states::GameState;

use self::camera::CursorPosition;

pub mod camera;
pub mod player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition(IVec2::ZERO));

        app.add_systems(
            OnEnter(GameState::InGame),
            (camera::spawn_camera, player::spawn_player).chain(),
        );

        app.add_systems(
            Update,
            (
                player::movement,
                player::update_positions,
                camera::follow_player,
                camera::update_cursor_position,
                camera::zoom,
                player::mouse_input,
                player::mouse_attack,
            )
                .run_if(in_state(GameState::InGame))
                .chain(),
        );
    }
}
