use mesh_data::MeshData;
use super::chunk::Chunk;
use super::direction::Direction;
use super::position::Position;
use super::voxel_geometry::{add_quad_triangles, create_vertex_position_face};

pub fn build_mesh(chunk: &Chunk) -> MeshData {
    let mut mesh_data = MeshData::new();

    for x in 0..chunk.size_x {
        for y in 0..chunk.size_y {
            for z in 0..chunk.size_z {
                if chunk.is_solid(x as i64, y as i64, z as i64) {
                    voxel_data(chunk, x as i64, y as i64, z as i64, &mut mesh_data);
                }
            }
        }
    }

    mesh_data
}

fn voxel_data<'a>(chunk: &Chunk, x: i64, y: i64, z: i64, mesh_data: &'a mut MeshData) {
    let position: Position = Position::new(x, y, z);

    for direction in Direction::iterator() {
        let touching_pos: Position = position.add_direction(direction.clone());

        // Build the face if there is no touching cube or if is the side of the chunk.
        if chunk.is_position_out_of_bound(&touching_pos) || chunk.is_position_air(&touching_pos) {
            create_vertex_position_face(mesh_data, &position, direction);
            add_quad_triangles(mesh_data);

            // Vector2 colorUv = colorTexture.GetColorUV(
            //     chunk.GetVoxel(pos).color
            // );
            //
            // mesh_data.uv.Add(colorUv);
            // mesh_data.uv.Add(colorUv);
            // mesh_data.uv.Add(colorUv);
            // mesh_data.uv.Add(colorUv);
        }
    }
}
