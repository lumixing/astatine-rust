use bevy::prelude::*;

use super::{chunks::CHUNK_SIZE, storage::WORLD_CHUNK_SIZE};

#[derive(Component, Eq, Hash, Copy, Clone)]
pub struct ChunkPos(pub IVec2);

impl ChunkPos {
    pub fn in_bounds(&self) -> bool {
        self.0.x >= 0
            && self.0.x < WORLD_CHUNK_SIZE.x
            && self.0.y >= 0
            && self.0.y < WORLD_CHUNK_SIZE.y
    }

    pub fn from_block_pos(block_pos: IVec2) -> Self {
        Self(IVec2 {
            x: block_pos.x / CHUNK_SIZE,
            y: block_pos.y / CHUNK_SIZE,
        })
    }
}

impl PartialEq for ChunkPos {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

/// relative to chunk block_pos to linearized relative to chunk block_index
pub fn linearize(block_pos: IVec2) -> usize {
    (block_pos.x + CHUNK_SIZE * block_pos.y) as usize
}
