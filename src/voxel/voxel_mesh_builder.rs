use super::mesh_data::MeshData;
use super::chunk::Chunk;
use super::direction::Direction;
use super::position::Position;
use super::voxel_geometry::{add_quad_triangles, create_vertex_face};
// use mesh::Mesh;

pub fn build_mesh(chunk: &Chunk) -> MeshData {
    let mesh_data = MeshData::new();

    for x in 0..chunk.size_x {
        for y in 0..chunk.size_y {
            for z in 0..chunk.size_z {
                if chunk.is_solid(x as i64, y as i64, z as i64) {
                    // meshData = voxel_data(chunk, x, y, z, meshData);
                }
            }
        }
    }

    mesh_data
}

fn voxel_data<'a>(chunk: &Chunk, x: i64, y: i64, z: i64, mesh_data: &'a mut MeshData) -> &'a MeshData {
    let position: Position = Position::new(x, y, z);

    for direction in Direction::iterator() {
        let touching_pos: Position = position.add_direction(direction.clone());

        // Build the face if there is no touching cube or if is the side of the chunk.
        if chunk.is_position_out_of_bound(&touching_pos) || chunk.is_position_air(&touching_pos) {
            create_vertex_face(mesh_data, &position, direction);
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

    return mesh_data;
}

// public MeshData BuildMesh(Chunk chunk) {
//     MeshData meshData = new MeshData();
//
//     for (int x = 0; x < chunk.SizeX; x++) {
//         for (int y = 0; y < chunk.SizeY; y++) {
//             for (int z = 0; z < chunk.SizeZ; z++) {
//                 if (chunk.voxels[x, y, z].IsSolid) { // If it is solid
//                     meshData = Voxeldata(chunk, x, y, z, meshData);
//                 }
//             }
//         }
//     }
//
//     return meshData;
// }

// MeshData Voxeldata(Chunk chunk, int x, int y, int z, MeshData meshData) {
//     Pos pos = new Pos(x, y, z);
//
//     foreach (var dir in DirectionUtils.Directions) {
//         Pos touchingPos = pos + dir;
//
//         // Build the face if there is no touching cube or if is the side of the chunk.
//         if (chunk.IsOutOfRange(touchingPos) || chunk.GetVoxel(touchingPos).IsAir) {
//             VoxelGeometry.CreateVertexFace(meshData, pos, dir);
//             VoxelGeometry.AddQuadTriangles(meshData);
//
//             Vector2 colorUv = colorTexture.GetColorUV(
//                 chunk.GetVoxel(pos).color
//             );
//
//             meshData.uv.Add(colorUv);
//             meshData.uv.Add(colorUv);
//             meshData.uv.Add(colorUv);
//             meshData.uv.Add(colorUv);
//         }
//     }
//
//     return meshData;
// }
