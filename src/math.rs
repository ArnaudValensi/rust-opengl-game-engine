use cgmath::{Point3, Vector3, vec3};

#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    if value < min {
        min
    }
    else if value > max {
        max
    } else {
        value
    }
}

#[inline]
pub fn point_to_vector(point: Point3<f32>) -> Vector3<f32> {
    vec3(
       point.x,
       point.y,
       point.z,
   )
}