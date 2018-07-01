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
