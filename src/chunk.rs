#[derive(Debug)]
struct Size {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug)]
pub struct Chunk {
    size: Size,
    voxels: Vec<u8>,
}

impl Chunk {
    pub fn new(size_x: u8, size_y: u8, size_z: u8) -> Self {
        let size = (size_x * size_y * size_z) as usize;

        Chunk {
            size: Size { x: size_x, y: size_y, z: size_z },
            voxels: vec![0; size],
        }
    }

    pub fn set_voxel(&mut self, x: u8, y: u8, z: u8, i: u8) {
        let size = &self.size;

        if x >= size.x {
            println!("x must be lesser than size_x");
            ::std::process::exit(1);
        }

        if y >= size.y {
            println!("y must be lesser than size_y");
            ::std::process::exit(1);
        }

        if z >= size.z {
            println!("z must be lesser than size_z");
            ::std::process::exit(1);
        }

        let index = (z * size.x * size.y) + (y * size.x) + x;

        self.voxels[index as usize] = i;
    }
}
