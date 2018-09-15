use failure::Fail;
use mesh_data::MeshData;
use super::voxel_mesh_builder::build_mesh;
use super::voxel::is_solid;
use super::position::Position;

const ERROR_VOXEL_OUT_OF_BOUND: &str = "the position of the voxel you are trying to set is out of bound";

#[derive(Fail, Debug)]
#[fail(display = "The position of the voxel you are trying to set is out of bound")]
pub struct ChunkOutOfBoundError;

#[derive(Debug, Clone)]
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

    pub fn is_in_bound(&self, x: i64, y: i64, z: i64) -> bool {
        x >= 0 &&
        y >= 0 &&
        z >= 0 &&
        x < self.size_x as i64 &&
        y < self.size_y as i64 &&
        z < self.size_z as i64
    }

    pub fn is_out_of_bound(&self, x: i64, y: i64, z: i64) -> bool {
        !self.is_in_bound(x, y, z)
    }

    pub fn is_position_in_bound(&self, position: &Position) -> bool {
        self.is_in_bound(position.x, position.y, position.z)
    }

    pub fn is_position_out_of_bound(&self, position: &Position) -> bool {
        self.is_out_of_bound(position.x, position.y, position.z)
    }

    pub fn set_voxel(&mut self, x: i64, y: i64, z: i64, i: u8) -> Result<(), ChunkOutOfBoundError> {
        if self.is_out_of_bound(x, y, z) {
            return Err(ChunkOutOfBoundError);
        }

        let index = (z * self.size_x as i64 * self.size_y as i64) + (y * self.size_x as i64) + x;

        self.voxels[index as usize] = i;
        Ok(())
    }

    pub fn get_voxel(&self, x: i64, y: i64, z: i64) -> Result<u8, ChunkOutOfBoundError> {
        if self.is_out_of_bound(x, y, z) {
            return Err(ChunkOutOfBoundError);
        }

        let index = (z * self.size_x as i64 * self.size_y as i64) + (y * self.size_x as i64) + x;

        Ok(self.voxels[index as usize])
    }

    pub fn build_mesh(&self) -> MeshData {
        build_mesh(&self)
    }

    // TODO: Improve error handling here
    pub fn is_solid(&self, x: i64, y: i64, z: i64) -> bool {
        let voxel = self.get_voxel(x, y, z).expect(ERROR_VOXEL_OUT_OF_BOUND);

        is_solid(voxel)
    }

    pub fn is_air(&self, x: i64, y: i64, z: i64) -> bool {
        !self.is_solid(x, y, z)
    }

    pub fn is_position_solid(&self, position: &Position) -> bool {
        self.is_solid(position.x, position.y, position.z)
    }

    pub fn is_position_air(&self, position: &Position) -> bool {
        self.is_air(position.x, position.y, position.z)
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
