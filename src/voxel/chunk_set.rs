use super::chunk::Chunk;
use std::collections::HashMap;

const CHUNKSIZE:u8 = 16;

pub struct ChunkSet {
    size: u64,
    chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl ChunkSet {
    pub fn new(size: u64) -> Self {
        let chunks = HashMap::with_capacity((size * size) as usize);

        Self {
            size,
            chunks,
        }
    }

    pub fn create_chunk(&mut self, x: i32, y: i32, z: i32) {}

    pub fn destroy_chunk(&mut self, x: i32, y: i32, z: i32) {}

    pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> Option<&Chunk> {
        self.chunks.get(&(x, y, z))
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&u8> {
        Some(&0)
    }

    pub fn set_block(&self, x: i32, y: i32, z: i32, voxel: u8) {}
}

// public Chunk CreateChunk(BlockPos pos)
// public void DestroyChunk(BlockPos pos)
// public void DestroyAllChunks()
// public void ReturnChunkToPool(Chunk chunk)
// public Chunk GetChunk(BlockPos pos)
// public byte GetBlock(BlockPos pos)
// public void SetBlock(BlockPos pos, byte block, bool updateChunk = true, bool withEdge = false)
// public void UpdateAdjacentChunks(BlockPos pos)
