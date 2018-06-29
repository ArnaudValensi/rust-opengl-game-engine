// TODO:
// - rotate_around_world_space(&mut self, rotation: Vector3<f32>)
// - set_left/set_up
// - up

use specs::{Component, VecStorage};
use cgmath::{Point3, Vector3, Quaternion, Euler, Deg, Rad, Angle};
use cgmath::prelude::*;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
    // scale
}

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
        };

        println!("= forward: {:#?}", t.forward());

        t
    }

    pub fn forward(&self) -> Vector3<f32> {
        let forward = self.rotation * Vector3::unit_z();

        println!("forward: {:#?}", forward);

        forward
    }

    pub fn set_forward(&mut self, new_forward: Vector3<f32>) {
        let up = Vector3::unit_y();

        self.rotation = Quaternion::look_at(new_forward, up);
    }

    pub fn left(&self) -> Vector3<f32> {
        // (self.rotation * Vector3::unit_x()).normalize()
        let down = -Vector3::unit_y();

        self.forward().cross(down).normalize()
    }

    pub fn rotate(&mut self, rotation: Vector3<f32>) {
        println!("rotation: {:#?}", rotation);
        self.rotation = (Quaternion::from(Euler {
            x: Deg(rotation.x),
            y: Deg(rotation.y),
            z: Deg(rotation.z),
        }) * self.rotation).normalize();
    }

    pub fn get_local_euler_angles(&self) -> Vector3<f32> {
        let euler: Euler<Rad<f32>> = Euler::from(self.rotation);

        Vector3 {
            x: Deg::from(euler.x).0,
            y: Deg::from(euler.y).0,
            z: Deg::from(euler.z).0,
        }
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
