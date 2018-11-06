pub mod chunk;
pub mod direction;
pub mod position;
// TODO: remove pub?
pub mod voxel_mesh_builder;
mod voxel_geometry;

pub mod chunk_set;
pub use self::chunk_set::ChunkSet;

pub mod terrain;
pub use self::terrain::Terrain;

pub fn is_solid(voxel: u8) -> bool {
    voxel != 0
}
