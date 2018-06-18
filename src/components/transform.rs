use specs::{Component, VecStorage};
use cgmath::Vector3;

#[derive(Debug)]
pub struct Transform {
    position: Vector3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
