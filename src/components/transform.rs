use specs::{Component, VecStorage};
use cgmath::{Point3, Vector3, Quaternion, Euler, Deg};
use cgmath::prelude::*;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
    // scale
    // pub forward: Vector3<f32>,
}

// public Vector3 forward
// {
//      get { return this.rotation * Vector3.forward; }
//      set { this.rotation = Quaternion.LookRotation(value); }
// }
impl Transform {
    pub fn new(position: Point3<f32>) -> Self {
        let up = Vector3::unit_y();
        let forward = -Vector3::unit_z();
        let rotation = Quaternion::look_at(forward, up);

        println!("= rotation: {:?}", rotation);
        println!("= rotation: {:#?}", Euler::from(rotation));

        let t = Transform {
            position,
            rotation,
            // forward: -Vector3::unit_z(),
        };

        println!("= forward: {:#?}", t.forward());

        t
    }

    pub fn forward(&self) -> Vector3<f32> {
        let forward = self.rotation * Vector3::unit_z();

        println!("forward: {:#?}", forward);

        forward
    }

    pub fn right(&self) -> Vector3<f32> {
        let up = Vector3::unit_y();

        // TODO: implement like forward
        self.forward().cross(up).normalize()
    }

    pub fn rotate(&mut self, rotation: Vector3<f32>) {
        println!("rotation: {:#?}", rotation);
        self.rotation = self.rotation * Quaternion::from(Euler {
            x: Deg(rotation.x),
            y: Deg(rotation.y),
            z: Deg(rotation.z),
        });
    }

    // TODO: rotate_around_world_space(&mut self, rotation: Vector3<f32>)
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
