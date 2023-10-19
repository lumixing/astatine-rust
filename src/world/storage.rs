use bevy::{prelude::*, utils::HashMap, math::ivec2};
// use rand::prelude::*;

use super::position::{ChunkPos, WORLD_SIZE, CHUNK_SIZE, linearize};

#[derive(Resource)]
pub struct WorldStorage(HashMap<ChunkPos, ChunkData>);

impl WorldStorage {
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        for y in 0..WORLD_SIZE.y {
            for x in 0..WORLD_SIZE.x {
                let chunk_pos = ChunkPos(ivec2(x, y));
                hashmap.insert(chunk_pos, ChunkData::new());
            }
        }
        Self(hashmap)
    }

    pub fn get_chunk_data(&self, chunk_pos: ChunkPos) -> Option<&ChunkData> {
        self.0.get(&chunk_pos)
    }

    fn get_mut_chunk_data(&mut self, chunk_pos: ChunkPos) -> Option<&mut ChunkData> {
        self.0.get_mut(&chunk_pos)
    }

    pub fn set_block(&mut self, block_pos: IVec2, tile: u32) {
        let chunk_pos = ChunkPos::from_block_pos(block_pos);
        let Some(chunk_data) = self.get_mut_chunk_data(chunk_pos) else {
            warn!("could not set block at {} since there is no chunk data at {}", block_pos, chunk_pos.0);
            return;
        };
        let block_rel_pos = ivec2(block_pos.x % CHUNK_SIZE, block_pos.y % CHUNK_SIZE);
        chunk_data.set_block(block_rel_pos, tile);
    }
}

pub struct ChunkData {
    blocks: Vec<u32>
}

impl ChunkData {
    pub fn new() -> Self {
        // let mut rng = rand::thread_rng();
        Self {
            blocks: vec![0; (CHUNK_SIZE*CHUNK_SIZE) as usize]
            // blocks: (0..CHUNK_SIZE*CHUNK_SIZE).map(|_| rng.gen_range(2..4)).collect()
        }
    }

    pub fn get_block(&self, block_pos: IVec2) -> Option<u32> {
        // if !block_pos.is_relative_chunk_pos() { return None; };
        let lin = linearize(block_pos);
        Some(self.blocks[lin])
    }

    pub fn set_block(&mut self, block_pos: IVec2, tile: u32) {
        // if !block_pos.is_relative_chunk_pos() { return None; };
        let lin = linearize(block_pos);
        self.blocks[lin] = tile;
    }
}