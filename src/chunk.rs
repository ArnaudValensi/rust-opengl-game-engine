use errors::*;

#[derive(Debug)]
pub struct Chunk {
    size_x: u8,
    size_y: u8,
    size_z: u8,
    voxels: Vec<u8>,
}

impl Chunk {
    pub fn new(size_x: u8, size_y: u8, size_z: u8) -> Self {
        let size = (size_x * size_y * size_z) as usize;

        Chunk {
            size_x,
            size_y,
            size_z,
            voxels: vec![0; size],
        }
    }

    pub fn is_position_in_bound(&self, x: u8, y: u8, z: u8) -> bool {
        x < self.size_x && y < self.size_y && z < self.size_z
    }

    pub fn set_voxel(&mut self, x: u8, y: u8, z: u8, i: u8) -> Result<()> {
        if !self.is_position_in_bound(x, y, z) {
            bail!("the position of the voxel you are trying to set is out of bound");
        }

        let index = (z * self.size_x * self.size_y) + (y * self.size_x) + x;

        self.voxels[index as usize] = i;
        Ok(())
    }

    pub fn get_voxel(&self, x: u8, y: u8, z: u8) -> Result<u8> {
        if !self.is_position_in_bound(x, y, z) {
            bail!("the position of the voxel you are trying to set is out of bound");
        }

        let index = (z * self.size_x * self.size_y) + (y * self.size_x) + x;

        Ok(self.voxels[index as usize])
    }
}
