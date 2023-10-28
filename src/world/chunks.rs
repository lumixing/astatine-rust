use bevy::{prelude::*, utils::{HashMap, HashSet}, math::ivec2};
use bevy_ecs_tilemap::tiles::TileStorage;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::{Tilesets, Tileset};

use crate::player::player::Player;

use super::{position::ChunkPos, storage::{ChunkData, WorldStorage}, block::Block};

pub const BLOCK_SIZE: i32 = 8;
pub const CHUNK_SIZE: i32 = 32;

#[derive(Resource)]
pub struct Colls(pub HashSet<(IVec2, IVec2)>); // pos, size

#[derive(Resource, Default)]
pub struct LoadedChunks(HashMap<ChunkPos, (Entity, Entity)>);

#[derive(Event)]
pub struct ReloadChunks;

#[allow(dead_code)]
impl LoadedChunks {
    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<&(Entity, Entity)> {
        self.0.get(&chunk_pos)
    }

    pub fn add_chunk(&mut self, chunk_pos: ChunkPos, chunk_entity: Entity, wall_chunk_entity: Entity) {
        if !chunk_pos.in_bounds() {
            warn!("could not load chunk that is out of bounds: {}", chunk_pos.0);
            return;
        }
        self.0.insert(chunk_pos, (chunk_entity, wall_chunk_entity));
    }

    pub fn remove_all_chunks(&mut self) {
        self.0.clear();
    }
}

pub fn spawn_chunks_near_player(
    mut commands: Commands,
    tilesets: Tilesets,
    mut loaded_chunks: ResMut<LoadedChunks>,
    mut colls: ResMut<Colls>,
    world_storage: Res<WorldStorage>,
    player_query: Query<&ChunkPos, With<Player>>,
    reload_event: EventReader<ReloadChunks>,
) {
    if reload_event.is_empty() { return; };
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let Ok(player_chunk_pos) = player_query.get_single() else { return };

    despawn_all_chunks(&mut commands, &mut loaded_chunks, &mut colls);
    for y in -4..=4 { //1
        for x in -4..=4 { //2
            let chunk_pos_raw = ivec2(x + player_chunk_pos.0.x as i32, y + player_chunk_pos.0.y as i32);
            let chunk_pos = ChunkPos(chunk_pos_raw);
            if !chunk_pos.in_bounds() { continue; };
            // let chunk_pos = ChunkPos::new(chunk_pos_raw.x as u32, chunk_pos_raw.y as u32);
            let chunk_data = world_storage.get_chunk_data(chunk_pos).unwrap(); // else this if error
            let chunk_entity = spawn_chunk(&mut commands, tileset, chunk_pos, chunk_data).unwrap();
            let wall_chunk_entity = spawn_wall_chunk(&mut commands, tileset, chunk_pos, chunk_data).unwrap();
            add_colls(&mut colls, chunk_pos, chunk_data);
            loaded_chunks.add_chunk(chunk_pos, chunk_entity, wall_chunk_entity);
        }
    }
}

fn despawn_all_chunks(
    commands: &mut Commands,
    loaded_chunks: &mut ResMut<LoadedChunks>,
    colls: &mut ResMut<Colls>
) {
    for (_, (chunk_entity, wall_chunk_entity)) in loaded_chunks.0.iter() {
        commands.entity(*chunk_entity).despawn_recursive();
        commands.entity(*wall_chunk_entity).despawn_recursive();
    }
    loaded_chunks.0.clear();
    colls.0.clear();
}

// haha is this branched?
fn add_colls(
    colls: &mut ResMut<Colls>,
    chunk_pos: ChunkPos,
    chunk_data: &ChunkData,
) {
    if !chunk_pos.in_bounds() {
        warn!("tried to add colls out of bounds! not spawning ({})", chunk_pos.0);
        return;
    };

    let mut expanded: HashSet<IVec2> = HashSet::new();
    for y in 0..CHUNK_SIZE {
        let mut s = -1;
        let mut i = 0;

        // 1d greedy meshing for colls
        for x in 0..CHUNK_SIZE {
            let block = chunk_data.get_block(ivec2(x, y)).unwrap();

            if block != Block::Air { // if solid
                // if from expanded coll, continue
                if expanded.contains(&ivec2(x, y)) {
                    info!("{x},{y} is already expanded!");
                    if s != -1 { // if theres a start then end it
                        end(&mut s, y, chunk_pos, &mut i, colls, chunk_data, &mut expanded);
                    }
                    continue;
                }; 

                if s == -1 { // if no start, new start
                    s = x;
                }

                i += 1; // increase current coll
                info!("{x},{y} increasing size!1d");

                if x == CHUNK_SIZE-1 { // if on last block, treat as air (end and add coll)
                    /*
                    let pos = ivec2(s+chunk_pos.0.x*32,y+chunk_pos.0.y*32);
                    let size = ivec2(i, 1);
                    colls.0.insert((pos, size));
                    */
                    info!("{x},{y} ended, end of chunk!1d");
                    end(&mut s, y, chunk_pos, &mut i, colls, chunk_data, &mut expanded);
                }
            } else { // if air
                if i == 0 {
                    info!("{x},{y} ignoring, no start!");
                    continue;
                }; // if no start ignore

                // end and add coll
                /*
                let pos = ivec2(s+chunk_pos.0.x*32,y+chunk_pos.0.y*32);
                let size = ivec2(i, 1);
                colls.0.insert((pos, size));
                s = -1;
                i = 0;
                */
                info!("{x},{y} ended, air!");
                end(&mut s, y, chunk_pos, &mut i, colls, chunk_data, &mut expanded);
            }
        }
    }

    fn end(s: &mut i32, y: i32, chunk_pos: ChunkPos, i: &mut i32,colls: &mut ResMut<'_, Colls>, chunk_data: &ChunkData, expanded: &mut HashSet<IVec2>) {
        // end 1d coll
        let pos = ivec2(*s+chunk_pos.0.x*32,y+chunk_pos.0.y*32);
        let mut size = ivec2(*i, 1);
        // colls.0.insert((pos, size));
        // *s = -1;
        // *i = 0;

        // check if can expand
        if y == CHUNK_SIZE-1 {
            info!("cant expand, end of chunK!");
            return;
        }; // cant expand, end of chunk
        for x in *s..(*s+*i) {
            let cpos = ivec2(x, y+1);
            // info!("current pos: {cpos}");
            let block = chunk_data.get_block(cpos).unwrap();
            if block == Block::Air { // cant expand :(
                colls.0.insert((pos, size));
                *s = -1;
                *i = 0;
                info!("cant expand!");
                return;
            }
            // info!("added {cpos} to expanded");
            expanded.insert(cpos);
        }
        info!("expanding!");
        // can expand!
        size.y += 1;
        colls.0.insert((pos, size));
    }
}

// maybe merge both spawn chunks?
fn spawn_chunk(
    commands: &mut Commands,
    tileset: &Tileset,
    chunk_pos: ChunkPos,
    chunk_data: &ChunkData,
) -> Option<Entity> {
    if !chunk_pos.in_bounds() {
        warn!("tried to spawn chunk out of bounds! not spawning ({})", chunk_pos.0);
        return None;
    };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TilemapSize { x: CHUNK_SIZE as u32, y: CHUNK_SIZE as u32 });
    let tileset_handle = tileset.texture();
    let chunk_transform = Transform::from_translation(Vec3::new(
        (chunk_pos.0.x * CHUNK_SIZE * BLOCK_SIZE) as f32,
        (chunk_pos.0.y * CHUNK_SIZE * BLOCK_SIZE) as f32,
        0.0
    ));

    let chunk_entity = commands.entity(tilemap_entity)
        .with_children(|builder| {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let block = chunk_data.get_block(ivec2(x, y)).unwrap();
                    let (flip_x, flip_y) = if block.should_flip() {
                        chunk_data.get_flip(ivec2(x, y)).unwrap()
                    } else {
                        (false, false)
                    };

                    let tile_pos = TilePos { x: x as u32, y: y as u32 };
                    let tile_entity = builder.spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(block as u32),
                        tilemap_id: TilemapId(builder.parent_entity()),
                        flip: TileFlip {
                            x: flip_x,
                            y: flip_y,
                            ..default()
                        },
                        ..default()
                    }).id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert((
            TilemapBundle {
                transform: chunk_transform,
                storage: tile_storage,
                size: TilemapSize { x: CHUNK_SIZE as u32, y: CHUNK_SIZE as u32 },
                grid_size: TilemapGridSize { x: BLOCK_SIZE as f32, y: BLOCK_SIZE as f32 },
                tile_size: TilemapTileSize { x: BLOCK_SIZE as f32, y: BLOCK_SIZE as f32 },
                texture: TilemapTexture::Single(tileset_handle.clone()),
                ..default()
            },
            chunk_pos
        ))
        .id();
    Some(chunk_entity)
}

fn spawn_wall_chunk(
    commands: &mut Commands,
    tileset: &Tileset,
    chunk_pos: ChunkPos,
    chunk_data: &ChunkData,
) -> Option<Entity> {
    if !chunk_pos.in_bounds() {
        warn!("tried to spawn chunk out of bounds! not spawning ({})", chunk_pos.0);
        return None;
    };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TilemapSize { x: CHUNK_SIZE as u32, y: CHUNK_SIZE as u32 });
    let tileset_handle = tileset.texture();
    let chunk_transform = Transform::from_translation(Vec3::new(
        (chunk_pos.0.x * CHUNK_SIZE * BLOCK_SIZE) as f32,
        (chunk_pos.0.y * CHUNK_SIZE * BLOCK_SIZE) as f32,
        -1.0
    ));

    let chunk_entity = commands.entity(tilemap_entity)
        .with_children(|builder| {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let front = chunk_data.get_block(ivec2(x, y)).unwrap();

                    if front != Block::Air { continue; };

                    let block = chunk_data.get_wall(ivec2(x, y)).unwrap();
                    let (flip_x, flip_y) = if block.should_flip() {
                        chunk_data.get_flip(ivec2(x, y)).unwrap()
                    } else {
                        (false, false)
                    };
                    
                    let tile_pos = TilePos { x: x as u32, y: y as u32 };
                    let tile_entity = builder.spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(block as u32),
                        tilemap_id: TilemapId(builder.parent_entity()),
                        flip: TileFlip {
                            x: flip_x,
                            y: flip_y,
                            ..default()
                        },
                        color: TileColor(Color::DARK_GRAY),
                        ..default()
                    }).id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert((
            TilemapBundle {
                transform: chunk_transform,
                storage: tile_storage,
                size: TilemapSize { x: CHUNK_SIZE as u32, y: CHUNK_SIZE as u32 },
                grid_size: TilemapGridSize { x: BLOCK_SIZE as f32, y: BLOCK_SIZE as f32 },
                tile_size: TilemapTileSize { x: BLOCK_SIZE as f32, y: BLOCK_SIZE as f32 },
                texture: TilemapTexture::Single(tileset_handle.clone()),
                ..default()
            },
            chunk_pos
        ))
        .id();
    Some(chunk_entity)
}