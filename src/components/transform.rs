use specs::{Component, VecStorage};
use cgmath::{Point3, Vector3};
use cgmath::prelude::*;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Point3<f32>,
    pub forward: Vector3<f32>,
}

impl Transform {
    pub fn right(&self) -> Vector3<f32> {
        let up = Vector3::unit_y();

        self.forward.cross(up).normalize()
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
