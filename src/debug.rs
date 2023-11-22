use bevy::{prelude::*, math::vec2};
use bevy_egui::{EguiContexts, egui};

use crate::{physics::Velocity, player::{player::Player, camera::CursorPosition}, world::chunks::Colls, entities::item::Item};

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

#[allow(dead_code)]
pub fn draw_colls(
    mut gizmos: Gizmos,
    colls: Res<Colls>
) {
    for (_, hashset) in colls.0.iter() {
        for (pos, len) in hashset.iter() {
            let npos = Vec2 {
                x: pos.x as f32 * 8.0 + *len as f32 * 4.0 - 4.0,
                y: pos.y as f32 * 8.0,
            };
            gizmos.rect_2d(npos, 0.0, vec2(8.0 * *len as f32, 8.0), Color::GREEN);
            // gizmos.rect_2d(pos.as_vec2() * 8.0, 0.0, vec2(8.0 * *len as f32, 8.0), Color::GREEN);
        }
    }
}

pub fn debug_text(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, &Velocity, &Player)>,
    item_query: Query<With<Item>>,
    colls: Res<Colls>,
    time: Res<Time>,
    cursor_pos: Res<CursorPosition>
) {
    let (transform, velocity, _player) = player_query.single();
    let mut coll_count = 0;
    for (_, hashset) in colls.0.iter() {
        coll_count += hashset.len();
    }

    egui::Window::new("debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("{}fps / {}ms", (1.0/time.delta_seconds()).floor(), (time.delta_seconds()*1000.0).floor()));
        ui.label(format!("wpos: {}", transform.translation.truncate().floor()));
        ui.label(format!("cpos: {}", cursor_pos.0));
        ui.label(format!("vel: {}", velocity.0));
        ui.label(format!("col: {}", coll_count));
        ui.label(format!("items: {}", item_query.iter().count()));
    });
}