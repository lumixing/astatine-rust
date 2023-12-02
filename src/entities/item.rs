use bevy::{prelude::*, sprite::collide_aabb::collide};

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

pub fn spawn(mut commands: Commands, mut item_event: EventReader<SpawnItem>) {
    for ev in item_event.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::PINK,
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
        ));
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
