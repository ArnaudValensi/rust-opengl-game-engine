use super::mesh_data::MeshData;
use super::chunk::Chunk;
// use mesh::Mesh;

pub fn build_mesh(chunk: &Chunk) -> MeshData {
    let mesh_data = MeshData {

    };

    for x in 0..chunk.size_x {
        for y in 0..chunk.size_y {
            for z in 0..chunk.size_z {
                if chunk.is_solid(x, y, z) {
                    // meshData = Voxeldata(chunk, x, y, z, meshData);
                }
            }
        }
    }

    mesh_data
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
