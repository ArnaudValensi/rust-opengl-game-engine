use errors::*;
use voxel::mesh_data::MeshData;
use voxel::voxel_mesh_builder::build_mesh;
use super::voxel::is_solid;

const ERROR_VOXEL_OUT_OF_BOUND: &str = "the position of the voxel you are trying to set is out of bound";

#[derive(Debug)]
pub struct Chunk {
    pub size_x: u8,
    pub size_y: u8,
    pub size_z: u8,
    pub voxels: Vec<u8>,
    mesh_data: Option<MeshData>,
}

// TODO:
// public Voxel GetVoxelBasedOnPlan(int x, int y, int layer, Direction direction)
// public int SizeXBasedOnPlan(Direction direction)
// public int SizeYBasedOnPlan(Direction direction)
// public int SizeZBasedOnPlan(Direction direction)
impl Chunk {
    pub fn new(size_x: u8, size_y: u8, size_z: u8) -> Self {
        let size = (size_x * size_y * size_z) as usize;

        Chunk {
            size_x,
            size_y,
            size_z,
            voxels: vec![0; size],
            mesh_data: None,
        }
    }

    pub fn is_in_bound(&self, x: u8, y: u8, z: u8) -> bool {
        x < self.size_x && y < self.size_y && z < self.size_z
    }

    pub fn is_out_of_bound(&self, x: u8, y: u8, z: u8) -> bool {
        !self.is_in_bound(x, y, z)
    }

    pub fn set_voxel(&mut self, x: u8, y: u8, z: u8, i: u8) -> Result<()> {
        if self.is_out_of_bound(x, y, z) {
            bail!(ERROR_VOXEL_OUT_OF_BOUND);
        }

        let index = (z * self.size_x * self.size_y) + (y * self.size_x) + x;

        self.voxels[index as usize] = i;
        Ok(())
    }

    pub fn get_voxel(&self, x: u8, y: u8, z: u8) -> Result<u8> {
        if self.is_out_of_bound(x, y, z) {
            bail!(ERROR_VOXEL_OUT_OF_BOUND);
        }

        let index = (z * self.size_x * self.size_y) + (y * self.size_x) + x;

        Ok(self.voxels[index as usize])
    }

    pub fn build_mesh(&self) -> MeshData {
        build_mesh(&self)
    }

    pub fn is_solid(&self, x: u8, y: u8, z: u8) -> bool {
        let voxel = self.get_voxel(x, y, z).expect(ERROR_VOXEL_OUT_OF_BOUND);

        is_solid(voxel)
    }
}

#[cfg(test)]
mod tests {
    use voxel::chunk::Chunk;

    #[test]
    fn set_voxel() {
        let mut chunk = Chunk::new(2, 3, 4);

        let result1 = chunk.set_voxel(2, 0, 1, 1);
        let result2 = chunk.set_voxel(1, 0, 1, 1);

        assert!(result1.is_err(), "it should return an out of bound error");
        assert!(!result2.is_err(), "it should not return an error");
    }

    #[test]
    fn get_voxel() {
        let mut chunk = Chunk::new(2, 3, 4);

        chunk.set_voxel(1, 0, 1, 1).expect("could not set voxel");
        let voxel = chunk.get_voxel(1, 0, 1);

        assert_eq!(voxel.unwrap(), 1, "it should get the correct voxel");
    }
}
