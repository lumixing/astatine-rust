use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::Tilesets;

pub fn spawn_chunk(
    mut commands: Commands,
    tilesets: Tilesets
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let tilemap_entity = commands.spawn_empty().id();
    let tileset_handle = tileset.texture();
    let mut tile_storage = TileStorage::empty(TilemapSize { x: 32, y: 32 });

    commands.entity(tilemap_entity)
        .with_children(|builder| {
            for y in 0..32 {
                for x in 0..32 {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = builder.spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(1),
                        tilemap_id: TilemapId(builder.parent_entity()),
                        ..default()
                    }).id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert((
            TilemapBundle {
                // transform: chunk_transform,
                storage: tile_storage,
                size: TilemapSize { x: 32, y: 32 },
                grid_size: TilemapGridSize { x: 8.0, y: 8.0 },
                tile_size: TilemapTileSize { x: 8.0, y: 8.0 },
                texture: TilemapTexture::Single(tileset_handle.clone()),
                ..default()
            },
        ));
}