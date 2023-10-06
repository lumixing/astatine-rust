use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));

    commands.spawn((
        camera_bundle,
        PlayerCamera
    ));
}