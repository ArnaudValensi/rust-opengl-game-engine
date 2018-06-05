use super::direction::Direction;
use super::position::Position;
use mesh_data::MeshData;
use vertex::Vertex;
use cgmath::Vector3;

pub fn create_vertex_position_face(mesh_data: &mut MeshData, position: &Position, direction: &Direction) {
    match direction {
        &Direction::North => {
            let mut vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32
            );
            let mut vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );

            // TODO: vertex values are set twice, the default one, then the vertex.
            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.Position = vertex_position_0;
            vertex_1.Position = vertex_position_1;
            vertex_2.Position = vertex_position_2;
            vertex_3.Position = vertex_position_3;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        },
        &Direction::East => {
            let mut vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.Position = vertex_position_0;
            vertex_1.Position = vertex_position_1;
            vertex_2.Position = vertex_position_2;
            vertex_3.Position = vertex_position_3;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        },
        &Direction::South => {
            let mut vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.Position = vertex_position_0;
            vertex_1.Position = vertex_position_1;
            vertex_2.Position = vertex_position_2;
            vertex_3.Position = vertex_position_3;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        },
        &Direction::West => {
            let mut vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.Position = vertex_position_0;
            vertex_1.Position = vertex_position_1;
            vertex_2.Position = vertex_position_2;
            vertex_3.Position = vertex_position_3;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        },
        &Direction::Up => {
            let mut vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 + 0.5_f32,
                position.z as f32 - 0.5_f32,
            );

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.Position = vertex_position_0;
            vertex_1.Position = vertex_position_1;
            vertex_2.Position = vertex_position_2;
            vertex_3.Position = vertex_position_3;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        },
        &Direction::Down => {
            let mut vertex_position_0 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );
            let mut vertex_position_1 = Vector3::<f32>::new(
                position.x as f32 - 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_2 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 - 0.5_f32,
            );
            let mut vertex_position_3 = Vector3::<f32>::new(
                position.x as f32 + 0.5_f32,
                position.y as f32 - 0.5_f32,
                position.z as f32 + 0.5_f32,
            );

            let mut vertex_0 = Vertex::default();
            let mut vertex_1 = Vertex::default();
            let mut vertex_2 = Vertex::default();
            let mut vertex_3 = Vertex::default();

            vertex_0.Position = vertex_position_0;
            vertex_1.Position = vertex_position_1;
            vertex_2.Position = vertex_position_2;
            vertex_3.Position = vertex_position_3;

            mesh_data.vertices.push(vertex_0);
            mesh_data.vertices.push(vertex_1);
            mesh_data.vertices.push(vertex_2);
            mesh_data.vertices.push(vertex_3);
        },
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
