use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::prelude::*;

use crate::{
    physics::{Rigidbody, Velocity},
    world::block::Block, player::player::Player,
};

#[derive(Event)]
pub struct SpawnItem {
    pub position: Vec2,
    pub block: Block,
}

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct AnimationOffset(f32);

pub fn spawn(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut item_event: EventReader<SpawnItem>,
) {
    let mut rng = thread_rng();
    for ev in item_event.iter() {
        let texture_handle = asset_server.load(ev.block.texture_path());
        let entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform {
                    translation: ev.position.extend(20.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            },
            Rigidbody {
                grounded: false,
                friction: true
            },
            Item,
            Velocity(Vec2::ZERO),
        )).id();

        let child = commands.spawn((
            SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform {
                    scale: Vec3::splat(1.0/8.0), // NO IDEA DONT ASK
                    ..default()
                },
                ..default()
            },
            AnimationOffset(rng.gen_range(0.0..100.0))
        )).id();

        commands.entity(entity).push_children(&[child]);
    }
}

pub fn check_collisions(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    item_query: Query<(&Transform, Entity), With<Item>>,
) {
    let player_transform = player_query.single();
    for (item_transform, item_entity) in item_query.iter() {
        let c = collide(
            player_transform.translation,
            player_transform.scale.truncate(),
            item_transform.translation,
            item_transform.scale.truncate()
        );

        if c.is_some() {
            commands.entity(item_entity).despawn_recursive();
        }
    }
}

pub fn animate(
    item_query: Query<&Children, With<Item>>,
    mut child_query: Query<(&mut Transform, &AnimationOffset)>,
    time: Res<Time>
) {
    for children in item_query.iter() {
        for &child in children {
            let (mut transform, offset) = child_query.get_mut(child).unwrap();
            transform.translation.y = (time.elapsed_seconds() * 4.0 + offset.0).sin() * 0.2 + 0.2;
        }
    }
}