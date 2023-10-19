use bevy::{prelude::*, math::vec3};
use rand::prelude::*;

use crate::physics::{Rigidbody, Velocity};

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                ..default()
            },
            transform: Transform {
                translation: vec3(150.0, 7.0*32.0*8.0+100.0, 20.0),
                scale: vec3(8.0, 8.0, 8.0),
                ..default()
            },
            ..default()
        },
        Rigidbody,
        Velocity(Vec2::ZERO)
    ));
}

pub fn stress(mut commands: Commands) {
    let mut rng = thread_rng();
    for _ in 0..500 {
        let x = rng.gen_range(0.0..512.0);
        let y = rng.gen_range(100.0..500.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE,
                    ..default()
                },
                transform: Transform {
                    translation: vec3(x, 7.0*32.0*8.0+y, 20.0),
                    scale: vec3(8.0, 8.0, 8.0),
                    ..default()
                },
                ..default()
            },
            Rigidbody,
            Velocity(Vec2::ZERO)
        ));
    }
}