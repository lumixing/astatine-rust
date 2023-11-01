use bevy::prelude::*;

use crate::{
    physics::{Rigidbody, Velocity},
    world::block::Block,
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
            Rigidbody,
            Item,
            Velocity(Vec2::ZERO),
        ));
    }
}
