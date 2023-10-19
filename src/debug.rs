use bevy::{prelude::*, math::vec2};

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