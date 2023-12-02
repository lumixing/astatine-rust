use std::ops::Div;

use bevy::{math::vec3, prelude::*};

use crate::{
    entities::{arrow::SpawnArrow, item::SpawnItem},
    physics::{Rigidbody, Velocity},
    world::{
        block::Block,
        chunks::{ReloadChunk, ReloadChunks},
        position::ChunkPos,
        storage::WorldStorage,
    },
};

use super::camera::CursorPosition;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            transform: Transform {
                translation: vec3(3000.0, 5000.0, 20.0),
                // translation: vec3(100.0, 8.0*32.0*8.0, 20.0),
                scale: vec3(8.0, 16.0, 8.0),
                ..default()
            },
            ..default()
        },
        Player,
        Rigidbody {
            grounded: false,
            friction: false,
        },
        ChunkPos(IVec2::ZERO),
        Velocity(Vec2::ZERO),
    ));
}

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = player_query.single_mut();

    velocity.0.x = 0.0;
    // velocity.0.y = 0.0;
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
    mut player_query: Query<(&Transform, &mut ChunkPos), With<Player>>,
    mut reload_event: EventWriter<ReloadChunks>,
) {
    let (transform, mut chunk_pos) = player_query.single_mut();
    let translation = IVec2 {
        x: (transform.translation.x / 8.0) as i32,
        y: (transform.translation.y / 8.0) as i32,
    };

    let new_chunk_pos = ChunkPos::from_block_pos(translation);
    if *chunk_pos != new_chunk_pos {
        *chunk_pos = new_chunk_pos;
        reload_event.send(ReloadChunks);
    }
}

pub fn mouse_input(
    cursor_pos: Res<CursorPosition>,
    mouse_input: Res<Input<MouseButton>>,
    mut world_storage: ResMut<WorldStorage>,
    mut reload_event: EventWriter<ReloadChunk>,
    mut item_event: EventWriter<SpawnItem>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        let block = world_storage.get_block(cursor_pos.0).unwrap();
        if block == Block::Air {
            return;
        };
        let chunk_pos = ChunkPos::from_block_pos(cursor_pos.0);

        item_event.send(SpawnItem {
            // position: cursor_pos.0.as_vec2() * 8.0,
            position: Vec2 {
                x: (cursor_pos.0.x * 8) as f32,
                y: (cursor_pos.0.y * 8) as f32,
            },
            block, // hehe crash here!, fix l8r B)
        });
        world_storage.set_block(cursor_pos.0, Block::Air);
        reload_event.send(ReloadChunk(chunk_pos));
    } else if mouse_input.pressed(MouseButton::Right) {
        let chunk_pos = ChunkPos::from_block_pos(cursor_pos.0);
        world_storage.set_block(cursor_pos.0, Block::Dirt);
        reload_event.send(ReloadChunk(chunk_pos));
    }
}

pub fn mouse_attack(
    player_query: Query<&Transform, With<Player>>,
    cursor_pos: Res<CursorPosition>,
    mouse_input: Res<Input<MouseButton>>,
    mut arrow_event: EventWriter<SpawnArrow>,
) {
    if !mouse_input.just_pressed(MouseButton::Middle) {
        return;
    };

    let player_transform = player_query.single();
    let block_pos = player_transform.translation.truncate().as_ivec2().div(8);
    let diff_pos = cursor_pos.0 - block_pos;
    let norm = diff_pos.as_vec2().normalize();

    arrow_event.send(SpawnArrow {
        position: player_transform.translation.truncate(),
        velocity: norm * 256.0,
    });
}

