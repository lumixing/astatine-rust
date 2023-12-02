use std::ops::Div;

use bevy::{prelude::*, math::{vec3, ivec2}, utils::HashSet};

use crate::{physics::{Rigidbody, Velocity}, world::{storage::WorldStorage, chunks::ReloadChunk, block::Block, position::ChunkPos}};

use super::item::SpawnItem;

#[derive(Event)]
pub struct SpawnArrow {
    pub position: Vec2,
    pub velocity: Vec2
}

#[derive(Component)]
pub struct Arrow;

pub fn spawn(mut commands: Commands, mut item_event: EventReader<SpawnArrow>) {
    for ev in item_event.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    ..default()
                },
                transform: Transform {
                    translation: ev.position.extend(20.0),
                    scale: vec3(2.0, 8.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Rigidbody {
                grounded: false,
                friction: true
            },
            Arrow,
            Velocity(ev.velocity),
        ));
    }
}

pub fn rotate(
    mut q: Query<(&mut Transform, &Velocity, &Rigidbody), With<Arrow>>
) {
    for (mut transform, velocity, rigidbody) in q.iter_mut() {
        if !rigidbody.grounded {
            let angle = velocity.0.x.atan2(velocity.0.y);
            transform.rotation = Quat::from_rotation_z(-angle);
        }
    }
}

pub fn check_collisions(
    mut commands: Commands,
    q: Query<(Entity, &Rigidbody, &Transform), With<Arrow>>,
    mut world_storage: ResMut<WorldStorage>,
    mut reload_event: EventWriter<ReloadChunk>,
    mut item_event: EventWriter<SpawnItem>,
) {
    for (entity, rigidbody, transform) in q.iter() {
        if !rigidbody.grounded { continue; };

        let block_pos = transform.translation.truncate().div(8.0).as_ivec2();
        commands.entity(entity).despawn_recursive();

        let mut chunks_to_reload: HashSet<ChunkPos> = HashSet::new();

        for y in block_pos.y-2..block_pos.y+2 {
            for x in block_pos.x-2..block_pos.x+2 {
                let pos = ivec2(x, y);
                let block = world_storage.get_block(pos).unwrap();
                if block == Block::Air {
                    continue;
                };
                let chunk_pos = ChunkPos::from_block_pos(pos);

                item_event.send(SpawnItem {
                    position: Vec2 {
                        x: (pos.x * 8) as f32,
                        y: (pos.y * 8) as f32,
                    },
                    block, // hehe crash here!, fix l8r B)
                });
                world_storage.set_block(pos, Block::Air);
                chunks_to_reload.insert(chunk_pos);
            }
        }
        
        for chunk_pos in chunks_to_reload.iter() {
            reload_event.send(ReloadChunk(*chunk_pos));
        }
    }
}