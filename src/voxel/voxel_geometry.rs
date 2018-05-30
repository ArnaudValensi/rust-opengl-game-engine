use super::mesh_data::MeshData;
use super::direction::Direction;
use super::position::Position;
use cgmath::Vector3;

// TODO: a list followed by a to_array should be more efficient here
pub fn create_vertex_face(mesh_data: &mut MeshData, position: &Position, direction: &Direction) {
    match direction {
        &Direction::North => {
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 + 0.5_f32
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
        },
        &Direction::East => {
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
        },
        &Direction::South => {
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
        },
        &Direction::West => {
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
        },
        &Direction::Up => {
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 + 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
        },
        &Direction::Down => {
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 - 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 + 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
            mesh_data.vertices.push(
                Vector3::<f32>::new(
                    position.x as f32 - 0.5_f32,
                    position.y as f32 - 0.5_f32,
                    position.z as f32 + 0.5_f32,
                )
            );
        },

    }
}

pub fn add_quad_triangles(mesh_data: &MeshData) {

}

// public static void AddQuadTriangles(MeshData meshData) {
// 	var triangles = meshData.triangles;
// 	var vertices = meshData.vertices;
//
// 	triangles.Add(vertices.Count - 4);
// 	triangles.Add(vertices.Count - 3);
// 	triangles.Add(vertices.Count - 2);
//
// 	triangles.Add(vertices.Count - 4);
// 	triangles.Add(vertices.Count - 2);
// 	triangles.Add(vertices.Count - 1);
// }
