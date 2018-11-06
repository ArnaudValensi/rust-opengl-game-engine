extern crate noise;

use super::chunk_set::ChunkSet;
use self::noise::Perlin;
use self::noise::NoiseModule;

pub struct Terrain {
    chunk_set: ChunkSet,
}

impl Default for Terrain {
    fn default() -> Self {
        let chunk_set = ChunkSet::new(2);

        Self { chunk_set }
    }
}

impl Terrain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&mut self) {
        let chunk_set = &mut self.chunk_set;

        chunk_set.create_chunk(0, 0, 0);
        chunk_set.create_chunk(16, 0, 0);
        chunk_set.create_chunk(0, 0, 16);
        chunk_set.create_chunk(16, 0, 16);


        let perlin = Perlin::new();

        for z in 0..32 {
            for x in 0..32 {
                let height = perlin.get([x as f32 / 100.0, z as f32 / 100.0]);

                println!("x: {}, z: {}, height: {}", x, z, height);
            }
        }
    }
}
