use super::direction::Direction;
use super::position::Position;
use cgmath::Vector3;
use mesh_data::MeshData;
use vertex::Vertex;

pub fn create_vertex_position_face(
    mesh_data: &mut MeshData,
    position: &Position,
    color_index: u8,
    direction: Direction,
) {
    match direction {
        Direction::North => {
            let vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );

            let normal = Vector3::<f32>::new(0.0, 0.0, -1.0);

            // TODO: vertex values are set twice, the default one, then the vertex.
            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.position = vertex_position_0;
            vertex_1.position = vertex_position_1;
            vertex_2.position = vertex_position_2;
            vertex_3.position = vertex_position_3;

            vertex_0.color_index = color_index;
            vertex_1.color_index = color_index;
            vertex_2.color_index = color_index;
            vertex_3.color_index = color_index;

            vertex_0.normal = normal;
            vertex_1.normal = normal;
            vertex_2.normal = normal;
            vertex_3.normal = normal;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        }
        Direction::East => {
            let vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let normal = Vector3::<f32>::new(1.0, 0.0, 0.0);

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.position = vertex_position_0;
            vertex_1.position = vertex_position_1;
            vertex_2.position = vertex_position_2;
            vertex_3.position = vertex_position_3;

            vertex_0.color_index = color_index;
            vertex_1.color_index = color_index;
            vertex_2.color_index = color_index;
            vertex_3.color_index = color_index;

            vertex_0.normal = normal;
            vertex_1.normal = normal;
            vertex_2.normal = normal;
            vertex_3.normal = normal;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        }
        Direction::South => {
            let vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let normal = Vector3::<f32>::new(0.0, 0.0, 1.0);

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.position = vertex_position_0;
            vertex_1.position = vertex_position_1;
            vertex_2.position = vertex_position_2;
            vertex_3.position = vertex_position_3;

            vertex_0.color_index = color_index;
            vertex_1.color_index = color_index;
            vertex_2.color_index = color_index;
            vertex_3.color_index = color_index;

            vertex_0.normal = normal;
            vertex_1.normal = normal;
            vertex_2.normal = normal;
            vertex_3.normal = normal;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        }
        Direction::West => {
            let vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let normal = Vector3::<f32>::new(-1.0, 0.0, 0.0);

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.position = vertex_position_0;
            vertex_1.position = vertex_position_1;
            vertex_2.position = vertex_position_2;
            vertex_3.position = vertex_position_3;

            vertex_0.color_index = color_index;
            vertex_1.color_index = color_index;
            vertex_2.color_index = color_index;
            vertex_3.color_index = color_index;

            vertex_0.normal = normal;
            vertex_1.normal = normal;
            vertex_2.normal = normal;
            vertex_3.normal = normal;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        }
        Direction::Up => {
            let vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );

            let normal = Vector3::<f32>::new(0.0, 1.0, 0.0);

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.position = vertex_position_0;
            vertex_1.position = vertex_position_1;
            vertex_2.position = vertex_position_2;
            vertex_3.position = vertex_position_3;

            vertex_0.color_index = color_index;
            vertex_1.color_index = color_index;
            vertex_2.color_index = color_index;
            vertex_3.color_index = color_index;

            vertex_0.normal = normal;
            vertex_1.normal = normal;
            vertex_2.normal = normal;
            vertex_3.normal = normal;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        }
        Direction::Down => {
            let vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let normal = Vector3::<f32>::new(0.0, -1.0, 0.0);

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.position = vertex_position_0;
            vertex_1.position = vertex_position_1;
            vertex_2.position = vertex_position_2;
            vertex_3.position = vertex_position_3;

            vertex_0.color_index = color_index;
            vertex_1.color_index = color_index;
            vertex_2.color_index = color_index;
            vertex_3.color_index = color_index;

            vertex_0.normal = normal;
            vertex_1.normal = normal;
            vertex_2.normal = normal;
            vertex_3.normal = normal;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        }
    }
}

pub fn add_quad_triangles(mesh_data: &mut MeshData) {
    let vertices_count = mesh_data.vertices.len() as i32;

    mesh_data.indices.push(vertices_count - 4); // 0
    mesh_data.indices.push(vertices_count - 3); // 1
    mesh_data.indices.push(vertices_count - 2); // 2

    mesh_data.indices.push(vertices_count - 4); // 0
    mesh_data.indices.push(vertices_count - 2); // 2
    mesh_data.indices.push(vertices_count - 1); // 3
}
