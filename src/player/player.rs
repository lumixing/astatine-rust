use bevy::{prelude::*, math::vec3};

use crate::{world::{position::ChunkPos, chunks::ReloadChunks, storage::WorldStorage, block::Block}, physics::{Velocity, Rigidbody}, entities::item::SpawnItem};

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
                translation: vec3(32.0, 32.0, 20.0),
                // translation: vec3(3000.0, 3800.0, 20.0),
                // translation: vec3(100.0, 8.0*32.0*8.0, 20.0),
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
    velocity.0.y = 0.0;
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

pub fn reload_chunks(
    mut reload_event: EventWriter<ReloadChunks>
) {
    reload_event.send(ReloadChunks);
}

pub fn update_positions(
    mut player_query: Query<(&Transform, &mut ChunkPos), With<Player>>,
    mut reload_event: EventWriter<ReloadChunks>,
) {
    let (transform, mut chunk_pos) = player_query.single_mut();
    let translation = IVec2 {
        x: (transform.translation.x / 8.0) as i32,
        y: (transform.translation.y / 8.0) as i32
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
    mut reload_event: EventWriter<ReloadChunks>,
    mut item_event: EventWriter<SpawnItem>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        let Some(block) = world_storage.get_block(cursor_pos.0) else { return; };
        if block == Block::Air { return; };

        item_event.send(SpawnItem {
            // position: cursor_pos.0.as_vec2() * 8.0,
            position: Vec2 {
                x: (cursor_pos.0.x * 8) as f32,
                y: (cursor_pos.0.y * 8 + 8) as f32
            },
            block, // hehe crash here!, fix l8r B)
        });
        world_storage.set_block(cursor_pos.0, Block::Air);
        reload_event.send(ReloadChunks);
    } else if mouse_input.pressed(MouseButton::Right) {
        let Some(block) = world_storage.get_block(cursor_pos.0) else { return; };
        if block != Block::Air { return; };

        world_storage.set_block(cursor_pos.0, Block::Dirt);
        reload_event.send(ReloadChunks);
    }
}