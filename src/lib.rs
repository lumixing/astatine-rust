use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};
use states::GameState;

mod states;
mod player;
mod world;
mod debug;
mod entities;
mod physics;

pub fn app() -> App {
    let mut app = App::new();

    app.add_state::<GameState>();
    app.add_loading_state(LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::WorldGeneration));
    
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::Immediate,
                    mode: bevy::window::WindowMode::Windowed,
                    title: format!("astatine.rs"),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        world::WorldPlugin,
        player::PlayerPlugin,
        entities::EntitiesPlugin,
        physics::PhysicsPlugin,
    ));

    app.add_systems(Update, (   
        bevy::window::close_on_esc,
        debug::debug_text,
        // debug::chunk_borders,
        debug::draw_colls,
    ).run_if(in_state(GameState::InGame)));

    app
}