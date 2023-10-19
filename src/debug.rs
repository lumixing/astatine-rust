use bevy::{prelude::*, math::{vec2, Vec3Swizzles}};
use bevy_egui::{EguiContexts, egui};

use crate::{physics::Velocity, player::player::Player, world::chunks::Colls};

#[allow(dead_code)]

pub fn chunk_borders(
    mut gizmos: Gizmos
) {
    for x in -128..128 {
        gizmos.line_2d(vec2(32.0*8.0*x as f32 -4.0, -99999.0), vec2(32.0*8.0*x as f32 -4.0, 99999.0), Color::BLUE);
    }

    for y in -128..128 {
        gizmos.line_2d(vec2(-99999.0, 32.0*8.0*y as f32 -4.0), vec2(99999.0, 32.0*8.0*y as f32 -4.0), Color::BLUE);
    }
}

pub fn draw_colls(
    mut gizmos: Gizmos,
    colls: Res<Colls>
) {
    for c in colls.0.iter() {
        gizmos.rect_2d(c.as_vec2() * 8.0, 0.0, Vec2::splat(8.0), Color::GREEN);
    }
}

pub fn debug_text(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, &Velocity, &Player)>,
    colls: Res<Colls>
) {
    let (transform, velocity, _player) = player_query.single();

    egui::Window::new("astatine debug shit").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("world position: {}", transform.translation.xy()));
        ui.label(format!("velocity: {}", velocity.0));
        // ui.label(format!("grounded: {}", player.grounded));
        ui.label(format!("colls: {}", colls.0.iter().count()));
    });
}