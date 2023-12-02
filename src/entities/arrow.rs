use bevy::{prelude::*, math::vec3};

use crate::physics::{Rigidbody, Velocity};

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
