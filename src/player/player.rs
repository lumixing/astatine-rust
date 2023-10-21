use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, math::vec3};

use crate::{world::position::ChunkPos, physics::{Velocity, Rigidbody}};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));

    commands.spawn((
        camera_bundle,
        PlayerCamera
    ));
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            transform: Transform {
                translation: vec3(100.0, 8.0*32.0*8.0, 20.0),
                scale: vec3(8.0, 16.0, 8.0),
                ..default()
            },
            ..default()
        },
        Player,
        Rigidbody,
        ChunkPos(IVec2::ZERO),
        Velocity(Vec2::ZERO)
    ));
}

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = player_query.single_mut();

    velocity.0.x = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        velocity.0.x = -128.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        velocity.0.x = 128.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        velocity.0.y = 128.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        velocity.0.y = -128.0;
    }
}

pub fn update_positions(
    mut player_query: Query<(&Transform, &mut ChunkPos), With<Player>>
) {
    let (transform, mut chunk_pos) = player_query.single_mut();
    let translation = IVec2 {
        x: (transform.translation.x / 8.0) as i32,
        y: (transform.translation.y / 8.0) as i32
    };

    let new_chunk_pos = ChunkPos::from_block_pos(translation);
    if *chunk_pos != new_chunk_pos {
        *chunk_pos = new_chunk_pos;
    }
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform.translation;
}