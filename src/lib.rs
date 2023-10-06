use bevy::{prelude::*, utils::HashSet};
use bevy_asset_loader::{loading_state::{LoadingState, LoadingStateAppExt}, asset_collection::AssetCollection};
use bevy_egui::EguiPlugin;
use bevy_tileset::prelude::Tileset;
use states::GameState;

mod states;
mod player;
mod world;
mod debug;

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct TileTextures {
    #[asset(path = "world_tiles.ron")]
    tileset: Handle<Tileset>,
    #[asset(path = "world_walls.ron")]
    wallset: Handle<Tileset>,
}

#[derive(Resource)]
pub struct Colls(pub HashSet<IVec2>);

pub fn app() -> App {
    let mut app = App::new();
    
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    title: "astatine.rs".to_owned(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        bevy_ecs_tilemap::TilemapPlugin,
        bevy_tileset::prelude::TilesetPlugin::default(),
        EguiPlugin,
    ));

    app.add_state::<GameState>();
    app.add_loading_state(LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::InGame));
    app.add_collection_to_loading_state::<_, TileTextures>(GameState::AssetLoading);

    app.insert_resource(Colls(HashSet::new()));

    app.add_systems(OnEnter(GameState::InGame), (
        player::spawn_camera,
        player::spawn_player,
        world::spawn_chunk,
    ));

    app.add_systems(Update, (
        bevy::window::close_on_esc,
        player::player_input,
        player::apply_gravity,
        player::check_collision,
        player::apply_velocity,
        player::camera_follow_player,
        debug::debug_text,
    ).run_if(in_state(GameState::InGame)).chain());
    
    app
}