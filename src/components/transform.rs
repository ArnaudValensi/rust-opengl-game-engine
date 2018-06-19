use specs::{Component, VecStorage};
use cgmath::Point3;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Point3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
