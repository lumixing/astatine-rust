use std::ops::{Add, Div};

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

use super::player::Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Resource)]
pub struct CursorPosition(pub IVec2);

pub fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));

    commands.spawn((
        camera_bundle,
        PlayerCamera
    ));
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform.translation;
}

pub fn update_cursor_position(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    mut res_cursor_pos: ResMut<CursorPosition>
) {
    let window = windows.single();
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let (camera, camera_global_transform) = camera_query.single();

    let Some(world_cursor_pos) = camera.viewport_to_world_2d(camera_global_transform, cursor_pos) else { return; };
    let tile_cursor_pos = world_cursor_pos.as_ivec2().add(4).div(8); // change this if something breaks
    res_cursor_pos.0 = tile_cursor_pos;
}


pub fn zoom(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<&mut OrthographicProjection, With<PlayerCamera>>,
    time: Res<Time>
) {
    let mut proj = q.single_mut();
    let delta = time.delta_seconds();

    if keyboard_input.pressed(KeyCode::Minus) {
        proj.scale += 0.1 * delta;
    }
    if keyboard_input.pressed(KeyCode::Equals) {
        proj.scale -= 0.1 * delta;
    }
    if keyboard_input.pressed(KeyCode::Back) {
        proj.scale = 0.5;
    }
}