use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_egui::{EguiContexts, egui};

use crate::{player::{Player, Velocity}, Colls};

pub fn debug_text(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, &Velocity, &Player)>,
    colls: Res<Colls>
) {
    let (transform, velocity, player) = player_query.single();

    egui::Window::new("astatine debug shit").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("world position: {}", transform.translation.xy()));
        ui.label(format!("velocity: {}", velocity.0));
        ui.label(format!("grounded: {}", player.grounded));
        ui.label(format!("colls: {}", colls.0.iter().count()));
    });
}