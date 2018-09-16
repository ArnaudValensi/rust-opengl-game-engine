// TODO:
// - rotate_around_world_space(&mut self, rotation: Vector3<f32>)
// - set_left/set_up
// - up

use specs::{Component, VecStorage};
use cgmath::{Point3, Vector3, Quaternion, Matrix4};
use cgmath::prelude::*;
use math::point_to_vector;
use std::f32::consts::PI;
use std::f32;

const RAD_TO_DEG: f32 = 180.0 / PI as f32;
const DEG_TO_RAD: f32 = PI / 180.0 as f32;

#[derive(Debug, Clone)]
pub struct Transform {
    pub name: &'static str,
    pub local_position: Point3<f32>,
    pub local_rotation: Quaternion<f32>,
    // TODO: Add scale.
    pub local_matrix: Matrix4<f32>,
    pub world_matrix: Matrix4<f32>,
    pub is_dirty: bool,
}

impl Transform {
    pub fn new(local_position: Point3<f32>, name: &'static str) -> Self {
        let up = Vector3::unit_y();
        let forward = Vector3::unit_z();
        let local_rotation = Quaternion::look_at(forward, up);
        let local_rotation_matrix = Matrix4::from(local_rotation);

        let translation = Matrix4::from_translation(point_to_vector(local_position));
        let local_matrix: Matrix4<f32> = translation * local_rotation_matrix;
        // let local_matrix: Matrix4<f32> = local_rotation_matrix * translation;

        Transform {
            name,
            local_position,
            local_rotation,
            local_matrix,
            world_matrix: local_matrix,
            is_dirty: true,
        }
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.local_rotation * Vector3::unit_z()
    }

    pub fn set_forward(&mut self, new_forward: Vector3<f32>) {
        let up = Vector3::unit_y();

        self.local_rotation = Quaternion::look_at(new_forward, up);
        //self.is_local_position_changed = true;
    }

    pub fn left(&self) -> Vector3<f32> {
        // (self.rotation * Vector3::unit_x()).normalize()
        let down = -Vector3::unit_y();

        self.forward().cross(down).normalize()
    }

    /// If `set_position()` has been called this frame, the last function calling will set the
    /// positions.
    pub fn set_local_position(&mut self, position: Point3<f32>) {
        self.local_position = position;
        self.is_dirty = true;
    }

    pub fn add_to_local_position(&mut self, position: Vector3<f32>) {
        let current_position = self.local_position;

        self.set_local_position(current_position + position);
    }

    // NOTE: Inspired from:
    //       - https://stackoverflow.com/questions/12088610/conversion-between-euler-quaternion-like-in-unity3d-engine
    //       - https://gist.github.com/aeroson/043001ca12fe29ee911e
    pub fn to_euler_angles(&self) -> Vector3<f32> {
        let w = self.local_rotation.s;
        let x = self.local_rotation.v.x;
        let y = self.local_rotation.v.y;
        let z = self.local_rotation.v.z;
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

        normalize_angles_vector(v * RAD_TO_DEG)
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.local_rotation = euler_to_quaternion(x, y, z);
        self.is_dirty = true;
    }
}

fn normalize_angles_vector(angles: Vector3<f32>) -> Vector3<f32> {
    Vector3::<f32> {
        x: normalize_angle(angles.x),
        y: normalize_angle(angles.y),
        z: normalize_angle(angles.z),
    }
}

// TODO: Modulo?
fn normalize_angle(angle: f32) -> f32 {
    let mut new_angle = angle;

    while new_angle > 360.0 {
        new_angle -= 360.0;
    }

    while new_angle < 0.0 {
        new_angle += 360.0;
    }

    new_angle
}

fn euler_to_quaternion(x: f32, y: f32, z: f32) -> Quaternion<f32> {
    let pitch = f64::from(x * DEG_TO_RAD);
    let roll = f64::from(z * DEG_TO_RAD);
    let yaw = f64::from(y * DEG_TO_RAD);

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
