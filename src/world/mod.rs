use bevy::{prelude::*, utils::HashSet};
use bevy_asset_loader::prelude::{AssetCollection, LoadingStateAppExt};
use bevy_egui::EguiPlugin;
use bevy_tileset::prelude::Tileset;

use crate::states::GameState;

use self::{gen::generate, chunks::Colls};

pub(crate) mod position;
mod storage;
pub(crate) mod chunks;
mod gen;
mod block;

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct TileTextures {
    #[asset(path = "world_tiles.ron")]
    tileset: Handle<Tileset>,
    #[asset(path = "world_walls.ron")]
    wallset: Handle<Tileset>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy_ecs_tilemap::TilemapPlugin,
            bevy_tileset::prelude::TilesetPlugin::default(),
            EguiPlugin
        ));

        app.add_collection_to_loading_state::<_, TileTextures>(GameState::AssetLoading);
        app.init_resource::<chunks::LoadedChunks>(); 
        app.insert_resource(Colls(HashSet::new()));

        app.add_systems(OnEnter(GameState::WorldGeneration), generate);

        app.add_systems(Update, (
            chunks::spawn_chunks_near_player,
        ).run_if(in_state(GameState::InGame)));
    }
}