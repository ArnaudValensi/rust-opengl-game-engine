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

pub fn add_quad_triangles(mesh_data: &mut MeshData) {
    let vertices_count = mesh_data.vertices.len() as i32;

    mesh_data.triangles.push(vertices_count - 4);
    mesh_data.triangles.push(vertices_count - 3);
    mesh_data.triangles.push(vertices_count - 2);

    mesh_data.triangles.push(vertices_count - 4);
    mesh_data.triangles.push(vertices_count - 2);
    mesh_data.triangles.push(vertices_count - 1);
}
