use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, math::{vec3, vec2}, input::mouse::MouseWheel, sprite::collide_aabb::{collide, Collision}};

use crate::Colls;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Player {
    pub grounded: bool
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.54;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));

    commands.spawn((
        camera_bundle,
        PlayerCamera
    ));
}

pub fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                ..default()
            },
            transform: Transform {
                translation: vec3(100.0, 300.0, 8.0),
                scale: vec3(8.0, 16.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player {
            grounded: false
        },
        Velocity(Vec2::ZERO)
    ));
}

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut player_query: Query<(&mut Velocity, &mut Player)>,
    mut camera_query: Query<&mut OrthographicProjection, With<PlayerCamera>>
) {
    // translate camera
    let (mut player_velocity, mut player) = player_query.single_mut();
    let mut camera_projection = camera_query.single_mut();

    player_velocity.0.x = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        player_velocity.0.x = -50.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player_velocity.0.x = 50.0;
    }
    if (keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Space)) && player.grounded {
        player_velocity.0.y = 300.0;
        player.grounded = false;
    }

    if keyboard_input.pressed(KeyCode::E) {
        camera_projection.scale = 0.5;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        camera_projection.scale = 1.0;
    }

    // scroll zoom camera
    for ev in scroll_evr.iter() {
        if ev.y.is_sign_positive() {
            camera_projection.scale += 0.01;
        } else {
            camera_projection.scale -= 0.01;
        }
        info!("{}", camera_projection.scale);
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
}

pub fn apply_gravity(
    mut q: Query<&mut Velocity>
) {
    let mut velocity = q.single_mut();
    velocity.0.y += -20.0;
    velocity.0.x = velocity.0.x.clamp(-512.0, 512.0);
    velocity.0.y = velocity.0.y.clamp(-512.0, 512.0);
}

pub fn apply_velocity(
    mut q: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for (mut transform, velocity) in q.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * delta;
    }
}

pub fn check_collision(
    mut q: Query<(&mut Transform, &mut Velocity, Entity)>,
    mut player_queue: Query<&mut Player>,
    colls: Res<Colls>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for (mut transform, mut velocity, entity) in q.iter_mut() {
        let mut should_ground = false;

        for coll_transform in colls.0.iter() {
            // vertical collision
            let vertical_translation = transform.translation + vec3(0.0, velocity.0.y * delta, 0.0);
            let coll = collide(
                vertical_translation,
                transform.scale.truncate(),
                coll_transform.as_vec2().extend(0.0) * 8.0,
                vec2(8.0, 8.0),
            );

            if let Some(c) = coll {
                velocity.0.y = 0.0;
                if c == Collision::Top {
                    transform.translation.y = (coll_transform.y as f32 * 8.0) + 12.0;
                    should_ground = true;
                }
            }

            // horizontal collision
            let horizontal_translation = transform.translation + vec3(velocity.0.x * delta, 0.0, 0.0);
            let coll = collide(
                horizontal_translation,
                transform.scale.truncate(),
                coll_transform.as_vec2().extend(0.0) * 8.0,
                vec2(8.0, 8.0),
            );

            if let Some(c) = coll {
                velocity.0.x = 0.0;
                if c == Collision::Right {
                    transform.translation.x = (coll_transform.x as f32 * 8.0) + 8.0;
                } else if c == Collision::Left {
                    transform.translation.x = (coll_transform.x as f32 * 8.0) - 8.0;
                }
            }
        }

        if player_queue.contains(entity) {
            player_queue.get_mut(entity).unwrap().grounded = should_ground;
        }
    }
}