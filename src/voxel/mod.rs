pub mod chunk;
pub mod direction;
pub mod position;
// TODO: remove pub?
pub mod voxel_mesh_builder;
mod voxel_geometry;

pub fn is_solid(voxel: u8) -> bool {
    voxel != 0
}
