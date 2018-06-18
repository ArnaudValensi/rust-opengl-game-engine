use specs::{Component, VecStorage};
use material::Material;
use mesh::Mesh;

#[derive(Debug)]
pub struct MeshRender {
    material: Material,
    mesh: Mesh,
}

impl Component for MeshRender {
    type Storage = VecStorage<Self>;
}
