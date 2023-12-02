use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}, math::{vec3, vec2}};

use crate::{states::GameState, world::chunks::Colls};

// pub mod player;

pub struct PhysicsPlugin;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Rigidbody {
    pub grounded: bool,
    pub friction: bool
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
    //     app.add_systems(OnEnter(GameState::InGame), (
    //         player::spawn_camera,
    //         player::spawn_player,
    //     ).chain());

        app.add_systems(Update, (
            apply_gravity,
            check_collision,
            apply_velocity,
        ).run_if(in_state(GameState::InGame)).chain());
    }
}

pub fn apply_gravity(
    mut q: Query<&mut Velocity, With<Rigidbody>>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for mut velocity in q.iter_mut() {
        velocity.0.y += -512.0 * delta;
        velocity.0.x = velocity.0.x.clamp(-512.0, 512.0);
        velocity.0.y = velocity.0.y.clamp(-512.0, 512.0);
    }
}

fn apply_velocity(
    mut q: Query<(&mut Transform, &mut Velocity, &Rigidbody)>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for (mut transform, mut velocity, _rigidbody) in q.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * delta;
        // if rigidbody.friction {
        //     velocity.0.x *= 0.9;
        // }
        if velocity.0.x.abs() < 0.01 {
            velocity.0.x = 0.0;
        }
    }
}

pub fn check_collision(
    // mut commands: Commands,
    mut q: Query<(&mut Transform, &mut Velocity, Entity, &mut Rigidbody)>,
    // mut player_queue: Query<&mut Player>,
    colls: Res<Colls>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for (mut transform, mut velocity, _entity, mut rigidbody) in q.iter_mut() {
        let mut should_ground = false;

        for (_, hashset) in colls.0.iter() {
            for (coll_transform, coll_length) in hashset.iter() {
                // vertical collision
                let coll_pos = Vec3 {
                    x: coll_transform.x as f32 * 8.0 + *coll_length as f32 * 4.0 - 4.0,
                    y: coll_transform.y as f32 * 8.0,
                    z: 0.0 
                };
                let vertical_translation = transform.translation + vec3(0.0, velocity.0.y * delta, 0.0);
                let coll = collide(
                    vertical_translation,
                    transform.scale.truncate(),
                    // (coll_transform.as_vec2().extend(0.0) * 8.0) + *coll_length as f32 * 4.0,
                    coll_pos,
                    vec2(8.0 * *coll_length as f32, 8.0),
                );
    
                if let Some(c) = coll {
                    velocity.0.y = 0.0;
                    if rigidbody.friction {
                        velocity.0.x = 0.0;
                        // commands.entity(entity).remove::<Rigidbody>();
                    }
                    if c == Collision::Top {
                        transform.translation.y = (coll_transform.y as f32 * 8.0) + (transform.scale.y / 2.0 + 4.0);
                        should_ground = true;
                    }
                }
    
                // horizontal collision
                let horizontal_translation = transform.translation + vec3(velocity.0.x * delta, 0.0, 0.0);
                let coll = collide(
                    horizontal_translation,
                    transform.scale.truncate(),
                    coll_pos,
                    vec2(8.0 * *coll_length as f32, 8.0),
                );
    
                if let Some(c) = coll {
                    velocity.0.x = 0.0;
                    if rigidbody.friction {
                        velocity.0.y = 0.0;
                        // commands.entity(entity).remove::<Rigidbody>();
                    }
                    if c == Collision::Right {
                        transform.translation.x = (coll_transform.x as f32 * 8.0) + (*coll_length * 8) as f32;
                    } else if c == Collision::Left {
                        transform.translation.x = (coll_transform.x as f32 * 8.0) - 8.0;
                    }
                }
            }
        }

        rigidbody.grounded = should_ground;
        // if player_queue.contains(entity) {
            // player_queue.get_mut(entity).unwrap().grounded = should_ground;
        // }
    }
}
