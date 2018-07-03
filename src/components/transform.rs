// TODO:
// - rotate_around_world_space(&mut self, rotation: Vector3<f32>)
// - set_left/set_up
// - up

use specs::{Component, VecStorage};
use cgmath::{Point3, Vector3, Quaternion, Euler, Deg, Rad};
use cgmath::prelude::*;
use std::f32::consts::PI;
use std::f32;

const RAD_TO_DEG: f32 = 180.0 / PI as f32;
const DEG_TO_RAD: f32 = PI / 180.0 as f32;

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

    // NOTE: Inspired from:
    //       - https://stackoverflow.com/questions/12088610/conversion-between-euler-quaternion-like-in-unity3d-engine
    //       - https://gist.github.com/aeroson/043001ca12fe29ee911e
    pub fn to_euler_angles(&self) -> Vector3<f32> {
        let w = self.rotation.s;
        let x = self.rotation.v.x;
        let y = self.rotation.v.y;
        let z = self.rotation.v.z;
        let square_w = w * w;
        let square_x = x * x;
        let square_y = y * y;
        let square_z = z * z;

        let unit = square_x + square_y + square_z + square_w;
        let test = x * w - y * z;
        let mut v = Vector3::<f32>::new(0.0, 0.0, 0.0);

        if test > 0.4995_f32 * unit {
            v.y = 2_f32 * y.atan2(x);
            v.x = PI / 2.0;
            v.z = 0.0;
            return normalize_angles_vector(v * RAD_TO_DEG);
        }

        if test < -0.4995_f32 * unit {
            v.y = -2_f32 * y.atan2(x);
            v.x = -PI / 2.0;
            v.z = 0.0;
            return normalize_angles_vector(v * RAD_TO_DEG);
        }

        let q = Quaternion::new(y, w, z, x);
        v.y = (2_f32 * q.v.x * q.s + 2_f32 * q.v.y * q.v.z).atan2(1.0 - 2_f32 * (q.v.z * q.v.z + q.s * q.s)) as f32;     // Yaw
        v.x = (2_f32 * (q.v.x * q.v.z - q.s * q.v.y)).asin() as f32;                                                     // Pitch
        v.z = (2_f32 * q.v.x * q.v.y + 2_f32 * q.v.z * q.s).atan2(1.0 - 2_f32 * (q.v.y * q.v.y + q.v.z * q.v.z)) as f32; // Roll
        return normalize_angles_vector(v * RAD_TO_DEG);
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = euler_to_quaternion(x, y, z);
    }
}

fn normalize_angles_vector(angles: Vector3<f32>) -> Vector3<f32> {
    Vector3::<f32> {
        x: normalize_angle(angles.x),
        y: normalize_angle(angles.y),
        z: normalize_angle(angles.z),
    }
}

fn normalize_angle(angle: f32) -> f32 {
    let mut new_angle = angle;

    while new_angle > 360.0 {
        new_angle -= 360.0;
    }

    while new_angle < 0.0 {
        new_angle += 360.0;
    }

    return new_angle;
}

fn euler_to_quaternion(x: f32, y: f32, z: f32) -> Quaternion<f32> {
    let pitch = (x * DEG_TO_RAD) as f64;
    let roll = (z * DEG_TO_RAD) as f64;
    let yaw = (y * DEG_TO_RAD) as f64;

    let pitch_over_2: f64 = pitch * 0.5_f64;
    let roll_over_2: f64 = roll * 0.5_f64;
    let yaw_over_2: f64 = yaw * 0.5_f64;

    let cy: f64 = yaw_over_2.cos();
    let sy: f64 = yaw_over_2.sin();
    let cr: f64 = roll_over_2.cos();
    let sr: f64 = roll_over_2.sin();
    let cp: f64 = pitch_over_2.cos();
    let sp: f64 = pitch_over_2.sin();

    let w: f32 = (cy * cr * cp + sy * sr * sp) as f32;
    let x: f32 = (cy * cr * sp + sy * sr * cp) as f32;
    let y: f32 = (sy * cr * cp - cy * sr * sp) as f32;
    let z: f32 = (cy * sr * cp - sy * cr * sp) as f32;

    Quaternion::<f32>::new(w, x, y, z)
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
