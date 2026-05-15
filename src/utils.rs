use crate::vector3::Vector3;
use crate::vector3::dot;
use std::f32::consts::PI;

pub const INFINITY: f32 = f32::INFINITY;
pub const EPSILON: f32 = 1e-6;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

pub fn generate_random_float_unit() -> f32 {
    rand::random::<f32>()
}

pub fn generate_random_float_in_range(min: f32, max: f32) -> f32 {
    return min + (max - min) * generate_random_float_unit();
}

pub fn smaple_from_unit_square() -> Vector3 {
    return Vector3::new(
        generate_random_float_unit() - 0.5,
        generate_random_float_unit() - 0.5,
        0.0,
    );
}

pub fn display_progress(height: u32, y: u32) {
    eprintln!(
        "{}% Done.",
        (((height - y) as f32) / height as f32 * 100.0).round()
    );
}

pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
    return v - normal * 2.0 * dot(v, normal);
}
