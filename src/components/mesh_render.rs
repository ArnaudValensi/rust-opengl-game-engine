use specs::{Component, VecStorage};
use material::Material;
use mesh::Mesh;

#[derive(Debug)]
pub struct MeshRender {
    pub material: Material,
    pub mesh: Mesh,
}

impl Component for MeshRender {
    type Storage = VecStorage<Self>;
}
